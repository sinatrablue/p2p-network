use std::{error::Error, vec};

mod network;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // test values for now, probably command line arg later on
    let peers_file = String::from("peers.json");
    let listen_port = 6789;
    let mut target_outgoing_connections = Vec::new();
    let max_incoming_connections = 5;
    let max_simultaneous_outgoing_connection_attempts = 5;
    let max_simultaneous_incoming_connection_attempts = 5;
    let max_idle_peers = 5;
    let max_banned_peers = 2;
    let peer_file_dump_interval_seconds = 8;

    // launch network controller
    let mut net = network::controller::NetworkController::new(
        peers_file,
        listen_port,
        target_outgoing_connections,
        max_incoming_connections,
        max_simultaneous_outgoing_connection_attempts,
        max_simultaneous_incoming_connection_attempts,
        max_idle_peers,
        max_banned_peers,
        peer_file_dump_interval_seconds
    ).await?;

    loop {
        tokio::select! {
            evt = net.wait_event() => match evt {
                Ok(msg) => match msg {
                    network::controller::NetworkControllerEvent::CandidateConnection {ip, socket, is_outgoing} => {
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
