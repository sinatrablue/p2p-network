use chrono::{DateTime, Utc};

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
    pub async fn new(
        peers_file: String,
        listen_port: u16,
        target_outgoing_connections: Vec<NetworkController>,
        max_incoming_connections: u32,
        max_simultaneous_outgoing_connection_attempts: u32,
        max_simultaneous_incoming_connection_attempts: u32,
        max_idle_peers: u32,
        max_banned_peers: u32,
        peer_file_dump_interval_seconds: u16
    ) {
        println!("Creating NetworkController");
    }
}