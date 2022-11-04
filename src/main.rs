use app::error::Error;
use client::connection::Connection;

mod app;
mod client;

#[tokio::main]
async fn main() -> Result<(), Error>{
    let mut connection = Connection::new("http://168.119.172.64").await?;
    let response = connection.get_request().await?;
    
    println!("{}", String::from_utf8(response.body.unwrap()).unwrap());

    Ok(())
}
