#![allow(dead_code)]
#![allow(unused_assignments)]

use std::path::PathBuf;

use app::error::Error;
use client::{connection::Connection, request::Request, method::Method};
use tokio::{fs::OpenOptions, io::AsyncReadExt};

mod app;
mod client;

#[tokio::main]
async fn main() -> Result<(), Error> {
    match Connection::new("https://benyaamin.com/content/files/BenyaminEskandari.pdf?some=one&hello=world").await {
        Ok(mut connection) => {
            let path: PathBuf = format!("{}", connection.parsed_url.file.clone().unwrap()).into();
            connection.download(&path).await.unwrap();
        },
        Err(e) => println!("{}", e)
    }
    Ok(())
}

