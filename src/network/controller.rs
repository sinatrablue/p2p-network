use std::{error::Error, fmt::Display, collections::HashMap};

use chrono::{DateTime, Utc};
use serde_json::{self, Value};
use tokio::net::{TcpListener, TcpStream};

#[cfg(test)]
pub mod tests;

#[derive(Debug, PartialEq)]
pub enum Status {
    Idle,
    OutConnecting,
    OutHandshaking,
    OutAlive,
    InHandshaking,
    InAlive,
    Banned
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct NetworkController {
    pub status: Option<Status>,
    pub last_alive: Option<DateTime<Utc>>,
    pub last_failure: Option<DateTime<Utc>>,
    pub listen_port: i32,
    peers: HashMap::<String, NetworkController>
}

impl NetworkController {
    pub async fn new (
        peers_file: String,
        listen_port: i32,
        target_outgoing_connections: HashMap::<String, NetworkController>,
        max_incoming_connections: u32,
        max_simultaneous_outgoing_connection_attempts: u32,
        max_simultaneous_incoming_connection_attempts: u32,
        max_idle_peers: u32,
        max_banned_peers: u32,
        peer_file_dump_interval_seconds: u16
    ) -> Result<NetworkController, Box<dyn Error>> {
        println!(".+* Creating NetworkController *+.");

        let peers = serde_json::from_str::<Value>(
                std::fs::read_to_string(&peers_file)
                .unwrap()
                .as_str())
            .unwrap();
            
        println!("[NetworkController::new] Initial peers list : \n{}", serde_json::to_string_pretty(&peers).unwrap());
        
        let net = NetworkController {
            status: None,
            last_alive: None,
            last_failure: None,
            listen_port,
            peers
        };

        Ok(net)
    }

    pub async fn wait_event(&self) -> Result<TcpStream, Box<dyn Error>> {
        println!("[NetworkController::wait_event] Lauching wait");
        let adr_listener = format!("localhost{}", self.listen_port);
        let listener = TcpListener::bind(adr_listener).await?;
        
        let (socket, _) = listener.accept().await?;
        println!("[NetworkController::wait_event] Accepted !");
        Ok(socket)
    }

    pub async fn feedback_peer_alive(&mut self, ip: &String) -> Result<(), Box<dyn Error>> {
        self.peers[&ip]["status"] = Value::String(String::from(Status::InAlive.to_string()));
        self.peers[&ip]["last_alive"] = Value::String(chrono::offset::Utc::now().to_string());
        Ok(())
    }

    pub async fn feedback_peer_failed(&mut self, ip: &String) -> Result<(), Box<dyn Error>> {
        self.peers[&ip]["status"] = Value::String(String::from(Status::Idle.to_string()));
        self.peers[&ip]["last_failure"] = Value::String(chrono::offset::Utc::now().to_string());
        Ok(())
    }

    pub async fn feedback_peer_banned(&mut self, ip: &String) -> Result<(), Box<dyn Error>> {
        self.peers[&ip]["status"] = Value::String(String::from(Status::Banned.to_string()));
        self.peers[&ip]["last_failure"] = Value::String(chrono::offset::Utc::now().to_string());
        Ok(())
    }

    pub async fn feedback_peer_closed(&mut self, ip: &String) -> Result<(), Box<dyn Error>> {
        self.peers[&ip]["status"] = Value::String(String::from(Status::Idle.to_string()));
        Ok(())
    }
    
    // pub async fn feedback_peer_failed(&self, ip: String) -> Result<(), Box<dyn Error>> {
    //     self.peers[ip].status = Status::Idle;
    //     self.peers[ip].last_failure = chrono::offset::Utc::now();
    //     Ok(())
    // }
}
