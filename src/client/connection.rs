use std::{path::PathBuf, sync::{mpsc::{Sender, Receiver}, Arc}};

use rand::{thread_rng, Rng};
use tokio::{net::TcpStream, io::{AsyncWriteExt, AsyncSeekExt}, fs::OpenOptions, sync::Semaphore};

use crate::app::error::Error;
use super::{url::ParsedUrl, response::Response, method::Method, request::{self, Request}};


static SEM: Semaphore = Semaphore::const_new(0);

#[derive(Clone)]
pub struct Connection {
    pub parsed_url: ParsedUrl
}

impl Connection {
    
    pub async fn new(url: &str) -> Result<Connection, Error> {
        let parsed_url = ParsedUrl::from(url)?;
        
        Ok(
            Connection { parsed_url }
        )
    }

    pub async fn request(&self, request: Request) -> Result<Response, Error> {
        let mut stream = TcpStream::connect(
            format!("{}:{}", self.parsed_url.host, self.parsed_url.port)
        ).await?;

        stream.write_all(format!("{} {} HTTP/1.1\r\n", request.get_method(), self.parsed_url.path).as_bytes()).await?;
        stream.write_all(format!("HOST: {}\r\n", self.parsed_url.host).as_bytes()).await?;
        
        for header in request.get_headers() {
            stream.write_all(
                format!("{}: {}\r\n", header.0, header.1).as_bytes()
            ).await?;
        }

        if let Some(range) = request.get_range() {
            stream.write_all(
                format!("Range: bytes={}-{}\r\n", range.start, range.end).as_bytes()
            ).await?;
        }
        
        stream.write_all(b"Connection: Close\r\n").await?;
        stream.write_all(b"\r\n\r\n").await?;

        Ok(Response::new(&mut stream).await?)
    }

    pub async fn download(&self, path: &PathBuf) -> Result<(), Error> {
        let head_request = Request::new().set_method(Method::HEAD);
        let head_response = self.request(head_request).await?;
        let mut file_path = PathBuf::from(path.to_str().unwrap());
        let mut file_name = String::new();

        if file_path.is_dir() {
            if let Some(fname) = &self.parsed_url.file {
                file_name.push_str(fname);
            }else {
                if let Some(content_disposition) = head_response.headers.get("Content-Disposition") {
                    let fname = content_disposition.split("=").last().unwrap();
                    file_name.push_str(fname.trim_matches('"'));
                }else {
                    let fname: String = thread_rng()
                        .sample_iter(&rand::distributions::Alphanumeric)
                        .take(15)
                        .map(char::from)
                        .collect();
                    file_name.push_str(&fname);
                }
            }

            file_path = file_path.join(file_name);
        }

        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(&file_path)
            .await?;

        let content_length: usize = match head_response.headers.get("Content-Length") {
            Some(v) => v.trim().parse().unwrap(),
            None => 0,
        };
        let connection_count = 5;
        SEM.add_permits(connection_count);
        let each_segment = 500_000;

        if content_length != 0 && content_length > each_segment {
            let (sender, receiver): (Sender<Response>, Receiver<Response>) = std::sync::mpsc::channel();
            let mut steps_left = content_length / each_segment;
            let mut range = 0..each_segment;
            let arc_self = Arc::new(self.clone());

            tokio::spawn(async move {                
                while let Ok(permit) = SEM.acquire().await {
                    let current_range = range.clone();
                    let _sender = sender.clone();
                    let _self = Arc::clone(&arc_self);

                    tokio::spawn(async move {
                        println!("spawn");
                        let _permit = permit;
                        let request = Request::new().set_range(current_range.clone());
                        let mut response = _self.request(request).await.unwrap();
                        response.range = Some(current_range);

                        _sender.send(response).unwrap();
                    });

                    range = if range.end + each_segment > content_length {
                        range.end + 1..content_length
                    }else {
                        range.end + 1..range.end + each_segment
                    };

                    if steps_left == 0 {
                        break;
                    }
                    steps_left -= 1;
                }
            });

        while let Ok(response) = receiver.recv() {
            println!("received");
            file.seek(std::io::SeekFrom::Start(response.range.unwrap().start as u64)).await?;
            file.write_all(response.body.unwrap().as_slice()).await?;
        }

        }else {
            let get_request = Request::new();
            let get_response = self.request(get_request).await?;
            file.write_all(get_response.body.unwrap().as_slice()).await?;
        }

        println!("File downloaded: {}", file_path.to_str().unwrap());

        Ok(())
    }

}