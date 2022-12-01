use core::fmt;
use std::{error::Error, fmt::Display};

use chrono::{DateTime, Utc};
use json;

pub enum Status {
    Idle,
    OutConnecting,
    OutHandshaking,
    OutAlive,
    InHandshaking,
    InAlive,
    Banned
}

pub struct NetworkController {
    pub status: Status,
    pub last_alive: Option<DateTime<Utc>>,
    pub last_failure: Option<DateTime<Utc>>
}

impl NetworkController {
    pub async fn new (
        peers_file: String,
        listen_port: u16,
        target_outgoing_connections: Vec<NetworkController>,
        max_incoming_connections: u32,
        max_simultaneous_outgoing_connection_attempts: u32,
        max_simultaneous_incoming_connection_attempts: u32,
        max_idle_peers: u32,
        max_banned_peers: u32,
        peer_file_dump_interval_seconds: u16
    ) -> Result<NetworkController, Box<dyn Error>> {
        println!(".+* Creating NetworkController *+.");

        let mut peers_list = json::parse(
            std::fs::read_to_string(&peers_file)
                .unwrap()
                .as_str())
            .unwrap();
            
        println!("[NetworkController::new] Initial peers list : \n{}", peers_list);
        
        let net = NetworkController {
            status: Status::Idle,
            last_alive: None,
            last_failure: None
        };
        Ok(net)
    }
}