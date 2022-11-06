use app::error::Error;
use client::connection::Connection;
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

use crate::client::method::Method;

mod app;
mod client;

#[tokio::main]
async fn main() -> Result<(), Error> {
    match Connection::new("http://168.119.172.64/BenyaminEskandari.pdf").await {
        Ok(mut connection) => {
            let response = connection.request(Method::GET, None).await?;
            
            let path = std::env::current_dir().unwrap().join("resume.pdf");
            let mut file = OpenOptions::new().create(true).write(true).open(path).await?;
            file.write_all(response.body.unwrap().as_slice()).await?;
            println!("File Downloaded.");
        },
        Err(e) => println!("{}", e)
    }
    Ok(())
}
