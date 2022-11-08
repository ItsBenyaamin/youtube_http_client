use std::{collections::HashMap, path::PathBuf};

use rand::{thread_rng, Rng};
use tokio::{net::TcpStream, io::AsyncWriteExt, fs::OpenOptions};

use crate::app::error::Error;

use super::{url::ParsedUrl, response::Response, method::Method};


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

    pub async fn request(
        &mut self, 
        method: Method, 
        request_headers: Option<HashMap<String, String>>
    ) -> Result<Response, Error> {
        let mut stream = TcpStream::connect(
            format!("{}:{}", self.parsed_url.host, self.parsed_url.port)
        ).await?;

        stream.write_all(format!("{} {} HTTP/1.1\r\n", method, self.parsed_url.path).as_bytes()).await?;
        stream.write_all(format!("HOST: {}\r\n", self.parsed_url.host).as_bytes()).await?;
        
        if let Some(headers) = request_headers {
            for header in headers {
                stream.write_all(
                    format!("{}: {}", header.0, header.1).as_bytes()
                ).await?;
            }
        }
        
        stream.write_all(b"Connection: Close\r\n").await?;
        stream.write_all(b"\r\n\r\n").await?;

        Ok(Response::new(&mut stream).await?)
    }

    pub async fn download(&mut self, path: &PathBuf) -> Result<(), Error> {
        let head_request = self.request(Method::HEAD, None).await?;
        let mut file_path = PathBuf::from(path.to_str().unwrap());
        let mut file_name = String::new();

        if file_path.is_dir() {
            if let Some(fname) = &self.parsed_url.file {
                file_name.push_str(fname);
            }else {
                if let Some(content_disposition) = head_request.headers.get("Content-Disposition") {
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

        let get_request = self.request(Method::GET, None).await?;

        file.write_all(get_request.body.unwrap().as_slice()).await?;

        println!("File downloaded: {}", file_path.to_str().unwrap());

        Ok(())
    }

}