use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use std::error::Error;

pub async fn basic_tcp_client() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("localhost:8080").await?;
    println!("Attempted to connect to Server");
    let res_write_stream = stream.write(b"Welcome to Massa").await;

    println!("Wrote to stream with result => {}", res_write_stream.is_ok());
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    basic_tcp_client().await?;

    Ok(())
}