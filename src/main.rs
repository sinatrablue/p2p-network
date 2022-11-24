use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::{TcpStream, TcpListener};

use std::error::Error;

async fn basic_tcp_server() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("localhost:6789").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                let sock_read_size = socket.read(&mut buf).await.expect("Couldn't read from socket");
                if sock_read_size == 0 {
                    return;
                }

                socket.write_all(&buf[0..sock_read_size/2]).await.expect("Couldn't write to client sock");
            }
        });
    }
}

async fn basic_tcp_client() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("localhost:6789").await?;
    let res_write_stream = stream.write(b"Welcome to Massa").await;

    println!("Wrote to stream with result => {}", res_write_stream.is_ok());
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    basic_tcp_server().await?;
    basic_tcp_client().await?;

    Ok(())
}
