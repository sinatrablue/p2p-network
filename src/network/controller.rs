use std::io;

use chrono::{DateTime, Utc};
use json;

enum Status {
    Idle,
    OutConnecting,
    OutHandshaking,
    OutAlive,
    InHandshaking,
    InAlive,
    Banned
}

pub struct NetworkController {
    status: Status,
    last_alive: Option<DateTime<Utc>>,
    last_failure: Option<DateTime<Utc>>
}

impl NetworkController {
    pub async fn new (
        peers_file: String
        /*listen_port: u16,
        target_outgoing_connections: Vec<NetworkController>,
        max_incoming_connections: u32,
        max_simultaneous_outgoing_connection_attempts: u32,
        max_simultaneous_incoming_connection_attempts: u32,
        max_idle_peers: u32,
        max_banned_peers: u32,
        peer_file_dump_interval_seconds: u16*/
    ) {
        println!("Creating NetworkController");

        let mut peers_list = json::parse(
            std::fs::read_to_string(&peers_file)
                .unwrap()
                .as_str())
            .unwrap();
        println!("Peers list : \n{}", peers_list);
        
        let new_peer = json::parse("
            {
                \"id\": 2,
                \"ip\": \"localhost:6786\",
                \"status\": \"Idle\"
            }")
        .unwrap();
        peers_list.insert(peers_list.len().to_string().as_str(), new_peer).unwrap();
        println!("###############################");
        println!("Peers list now => \n{}", peers_list);

        std::fs::write(&peers_file, peers_list.to_string()).unwrap();
    }
}