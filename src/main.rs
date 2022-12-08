use std::{error::Error, collections::HashMap};

use chrono::Duration;
use tokio::net::TcpStream;

mod network;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let input_json_peer_file = match std::env::args().nth(1) {
        Some(s) => s,
        None => panic!("No input file !")
    };
    println!("Input file is <{}>", input_json_peer_file);
    
    // Convert the JSON object to a HashMap
    //let hash_for_peers = network::controller::io_json::import_peers_from_json(&input_json_peer_file)?;

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

    //let mut need_connect_retry = true;
    //let mut wakeup_interval = tokio::time::interval(std::time::Duration::from_millis(1000));

    loop {
        /*
        if need_connect_retry {
            for ip in net.peers.keys().clone() {
                let socket = TcpStream::connect(&ip).await?;
                match network::controller::NetworkController::perform_handshake(&ip, socket, true).await? {
                    network::controller::events::NetworkControllerEvent::HandshakeStatus::HandshakeSuccess => net.feedback_peer_alive(&ip).await?,
                    network::controller::events::NetworkControllerEvent::HandshakeStatus::HandshakeFailure => net.feedback_peer_failed(&ip).await?
                };
            }
            need_connect_retry = false;
        }
        ==> causes a borrow of net as mutable and immutable in the same scope which is forbidden, the peers ips must come from elsewhere
        */
        tokio::select! {
            evt = net.wait_event() => match evt {
                Ok(msg) => match msg {
                    network::controller::events::NetworkControllerEvent::CandidateConnection {ip, socket, is_outgoing} => {
                        match network::controller::NetworkController::perform_handshake(&ip, socket, is_outgoing).await? {
                            network::controller::events::NetworkControllerEvent::HandshakeStatus::HandshakeSuccess => net.feedback_peer_alive(&ip).await?,
                            network::controller::events::NetworkControllerEvent::HandshakeStatus::HandshakeFailure => net.feedback_peer_failed(&ip).await?
                        };
                                                
                        //  if handshake fails or the connection closes unexpectedly at any time, call net.feedback_peer_failed(ip).await; to signal NetworkController to set the peer status to Idle  (this should update last_failure)
                        
                        // once the handshake is done, we can use this peer socket in main.rs
                    }
                },
                Err(e) => return Err(e)
            },
            /*
            _ = wakeup_interval.tick() => {
                need_connect_retry = true;
            }
            */
        }


    }

}
