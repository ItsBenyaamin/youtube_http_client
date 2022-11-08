use std::collections::HashMap;
use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::app::error::Error;

#[derive(Debug)]
pub struct Response {
    pub version: String,
    pub status_code: usize,
    pub status_name: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>
}

impl Response {

    pub async fn new(stream: &mut TcpStream) -> Result<Response, Error> {
        let mut buff: Vec<u8> = vec![];
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut body: Option<Vec<u8>> = None;
        let mut response_info = String::new();
        let mut is_header_section = false;

        loop {
            match stream.read_u8().await {
                Ok(byte) => {
                    buff.push(byte);

                    if byte as char != '\n' {
                        continue;
                    }

                    if response_info.is_empty() {
                        response_info = String::from_utf8(buff[..buff.len() - 2].to_vec())?;
                        buff.clear();
                        is_header_section = true;
                        continue;
                    }

                    if is_header_section {
                        if buff.len() == 2 && buff[0] as char == '\r' {
                            is_header_section = false;
                            buff.clear();
                            continue;
                        }

                        let header_line = String::from_utf8(buff[..buff.len() - 2].to_vec())?;
                        buff.clear();

                        match header_line.split_once(":") {
                            Some(v) => headers.insert(v.0.to_string(), v.1.to_string()),
                            None => return Err(Error::HeaderParsingError),
                        };
                    }
                },
                Err(_) => break,
            }
        }

        let mut response_info_split = response_info.split(" ");

        let version = match response_info_split.next() {
            Some(v) => v.to_string(),
            None => return Err(Error::HeaderParsingError),
        };

        let status_code = match response_info_split.next() {
            Some(v) => v.parse().unwrap(),
            None => return Err(Error::HeaderParsingError),
        };

        let status_name = match response_info_split.next() {
            Some(v) => v.to_string(),
            None => return Err(Error::HeaderParsingError),
        };

        if buff.len() > 0 {
            body = Some(buff);
        }

        Ok(
            Response { version, status_code, status_name, headers, body }
        )
    }

}