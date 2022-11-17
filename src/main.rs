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
    match Connection::new("172.29.1.159").await {
        Ok(connection) => {
            let file_path: PathBuf = "C:\\Users\\BeNYaMiN\\Pictures\\thumbnail.png".into();
            let mut file = OpenOptions::new().read(true).open(file_path).await?;
            let mut file_bytes = vec![];
            file.read_to_end(&mut file_bytes).await?;

            let request = Request::new()
                .set_method(Method::POST)
                .multipart()
                .add_data("name", "Benyaamin")
                .add_data("age", 26)
                .add_data("single", true)
                .add_data("something", 55.5)
                .add_file("file", "lGiq_dhT_400x400.jpg", file_bytes);
            let response = connection.request(request).await?;
            println!(
                "{}", String::from_utf8(response.body.unwrap()).unwrap()
            )
        },
        Err(e) => println!("{}", e)
    }
    Ok(())
}

