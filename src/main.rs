use std::path::PathBuf;

use app::error::Error;
use client::connection::Connection;

mod app;
mod client;

#[tokio::main]
async fn main() -> Result<(), Error> {
    match Connection::new("http://168.119.172.64/Documents.rar").await {
        Ok(mut connection) => {
            let path = PathBuf::from("C:\\Users\\BeNYaMiN\\Desktop\\");
            connection.download(&path).await?;
        },
        Err(e) => println!("{}", e)
    }
    Ok(())
}
