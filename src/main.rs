use std::{error::Error, collections::HashMap};

mod network;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let input_json_peer_file = std::env::args().nth(1).unwrap();
    println!("Input file is <{}>", input_json_peer_file);
    // test values for now, probably command line arg later on

    let listen_port = 8080;
    let target_outgoing_connections = HashMap::<String, network::controller::NetworkController>::new();
    let max_incoming_connections = 5;
    let max_simultaneous_outgoing_connection_attempts = 5;
    let max_simultaneous_incoming_connection_attempts = 5;
    let max_idle_peers = 5;
    let max_banned_peers = 2;
    let peer_file_dump_interval_seconds = 8;

    // launch network controller
    let mut net = network::controller::NetworkController::new(
        input_json_peer_file,
        listen_port,
        target_outgoing_connections,
        max_incoming_connections,
        max_simultaneous_outgoing_connection_attempts,
        max_simultaneous_incoming_connection_attempts,
        max_idle_peers,
        max_banned_peers,
        peer_file_dump_interval_seconds
    ).await?;
    println!("<net> status => {:?}", net.status);

    loop {
        tokio::select! {
            evt = net.wait_event() => match evt {
                Ok(msg) => match msg {
                    network::controller::events::NetworkControllerEvent::CandidateConnection {ip, socket, is_outgoing} => {
                        network::controller::NetworkController::perform_handshake(ip, socket, is_outgoing).await?;
                        // ip is the peer ip, and socket is a tokio TCPStream
                        // triggered when a new TCP connection with a peer is established
                        // is_outgoing is true if our node has connected to the peer node
                        // is_outgoing is false if the peer node has connected to our node
                        
                        // here, a handshake must be performed by reading/writing data to socket
                        //  if the handshake succesds, call net.feedback_peer_alive(ip).await; to signal NetworkController to set the peer in InAlive or OutAlive state (this should update last_alive)
                        //  if handshake fails or the connection closes unexpectedly at any time, call net.feedback_peer_failed(ip).await; to signal NetworkController to set the peer status to Idle  (this should update last_failure)
                        
                        // once the handshake is done, we can use this peer socket in main.rs
                    }
                },
                Err(e) => return Err(e)
            }
        }
    }

}
