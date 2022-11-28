use std::error::Error;

//use network::tcp_client;
use network::tcp_server;

pub mod network;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tcp_server::basic_tcp_server().await?;
    //tcp_client::basic_tcp_client().await?;

    Ok(())
}
