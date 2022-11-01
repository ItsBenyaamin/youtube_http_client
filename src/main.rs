use app::error::Error;
use client::connection::Connection;

mod app;
mod client;

#[tokio::main]
async fn main() -> Result<(), Error>{
    let mut connection = Connection::new("http://168.119.172.64").await?;
    connection.head_request().await?;
    Ok(())
}
