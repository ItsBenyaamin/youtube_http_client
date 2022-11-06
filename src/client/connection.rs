use std::collections::HashMap;

use tokio::{net::TcpStream, io::AsyncWriteExt};

use crate::app::error::Error;

use super::{url::ParsedUrl, response::Response, method::Method};


pub struct Connection {
    pub parsed_url: ParsedUrl,
    pub stream: TcpStream
}

impl Connection {
    
    pub async fn new(url: &str) -> Result<Connection, Error> {
        let parsed_url = ParsedUrl::from(url)?;
        let stream = TcpStream::connect(
            format!("{}:{}", parsed_url.host, parsed_url.port)
        ).await?;
        Ok(
            Connection { parsed_url, stream }
        )
    }

    pub async fn request(
        &mut self, 
        method: Method, 
        request_headers: Option<HashMap<String, String>>
    ) -> Result<Response, Error> {
        self.stream.write_all(format!("{} {} HTTP/1.1\r\n", method, self.parsed_url.path).as_bytes()).await?;
        self.stream.write_all(format!("HOST: {}\r\n", self.parsed_url.host).as_bytes()).await?;
        
        if let Some(headers) = request_headers {
            for header in headers {
                self.stream.write_all(
                    format!("{}: {}", header.0, header.1).as_bytes()
                ).await?;
            }
        }
        
        self.stream.write_all(b"Connection: Close\r\n").await?;
        self.stream.write_all(b"\r\n\r\n").await?;

        Ok(Response::new(self).await?)
    }

}