use tokio::{net::TcpStream, io::AsyncWriteExt};

use crate::app::error::Error;

use super::{url::ParsedUrl, response::Response};


pub struct Connection {
    pub parsed_url: ParsedUrl,
    pub stream: TcpStream
}

impl Connection {
    
    pub async fn new(url: &str) -> Result<Connection, Error> {
        let parsed_url = ParsedUrl::from(url)?;
        let stream = TcpStream::connect(format!("{}:80", parsed_url.host)).await?;
        Ok(
            Connection { parsed_url, stream }
        )
    }

    pub async fn head_request(&mut self) -> Result<Response, Error> {
        self.stream.write_all(format!("HEAD {} HTTP/1.1\r\n", self.parsed_url.path).as_bytes()).await?;
        self.stream.write_all(format!("HOST: {}\r\n", self.parsed_url.host).as_bytes()).await?;
        
        self.stream.write_all(b"Connection: Close\r\n").await?;
        self.stream.write_all(b"\r\n\r\n").await?;

        Ok(Response::new(self).await?)
    }

    pub async fn get_request(&mut self) -> Result<Response, Error> {
        self.stream.write_all(format!("GET {} HTTP/1.1\r\n", self.parsed_url.path).as_bytes()).await?;
        self.stream.write_all(format!("HOST: {}\r\n", self.parsed_url.host).as_bytes()).await?;
        
        self.stream.write_all(b"Connection: Close\r\n").await?;
        self.stream.write_all(b"\r\n\r\n").await?;

        Ok(Response::new(self).await?)
    }

}