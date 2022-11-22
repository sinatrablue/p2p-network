use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("localhost:6789").await?;
    let resWriteSteam = stream.write(b"Welcome to Massa").await;

    Ok(())
}
