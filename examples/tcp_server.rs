use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::TcpListener;

use std::error::Error;
use std::str;

pub async fn basic_tcp_server() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("localhost:6789").await?;
    println!("TCP Server listening ...");

    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("Accepted !");

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                let sock_read_size = socket.read(&mut buf).await.expect("Couldn't read from socket");
                if sock_read_size == 0 {
                    return;
                }

                println!("Received => {}", match str::from_utf8(&buf) {
                    Ok(s) => s,
                    Err(e) => panic!("Invalid buffer conversion => {}", e),
                });

                socket.write_all(&buf[0..sock_read_size/2]).await.expect("Couldn't write to client sock");
            }
        });
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    basic_tcp_server().await?;

    Ok(())
}