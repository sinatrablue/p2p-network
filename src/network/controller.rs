use std::{error::Error, fmt::Display, collections::HashMap, str::{from_utf8, FromStr}};

use chrono::{DateTime, Utc};
use tokio::{net::{TcpListener, TcpStream}, io::{AsyncWriteExt, AsyncReadExt}};

pub mod io_json;
pub mod events;
use events::NetworkControllerEvent::{CandidateConnection, HandshakeStatus};
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

impl FromStr for Status {

    type Err = ();

    fn from_str(input: &str) -> Result<Status, Self::Err> {
        match input {
            "Idle"              => Ok(Status::Idle),
            "OutConnecting"     => Ok(Status::OutConnecting),
            "OutHandshaking"    => Ok(Status::OutHandshaking),
            "OutAlive"          => Ok(Status::OutAlive),
            "InHandshaking"     => Ok(Status::InHandshaking),
            "InAlive"           => Ok(Status::InAlive),
            "Banned"            => Ok(Status::Banned),
            _                   => Err(()),
        }
    }
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
    pub listen_port: u16,
    pub max_incoming_connections: u32,
    pub max_simultaneous_outgoing_connection_attempts: u32,
    pub max_simultaneous_incoming_connection_attempts: u32,
    pub max_idle_peers: u32,
    pub max_banned_peers: u32,
    pub peers: HashMap::<String, NetworkController>
}

impl NetworkController {
    pub async fn new (
        peers_file: String,
        listen_port: u16,
        target_outgoing_connections: HashMap::<String, NetworkController>,
        max_incoming_connections: u32,
        max_simultaneous_outgoing_connection_attempts: u32,
        max_simultaneous_incoming_connection_attempts: u32,
        max_idle_peers: u32,
        max_banned_peers: u32,
        peer_file_dump_interval_seconds: u16
    ) -> Result<NetworkController, Box<dyn Error>> {
        println!(".+* Creating NetworkController *+.");

        let net = NetworkController {
            status: None,
            last_alive: None,
            last_failure: None,
            listen_port,
            max_incoming_connections,
            max_simultaneous_outgoing_connection_attempts,
            max_simultaneous_incoming_connection_attempts,
            max_idle_peers,
            max_banned_peers,    
            peers: target_outgoing_connections
        };

        /*
        loop {
            tokio::time::timeout(tokio::time::Duration::new(peer_file_dump_interval_seconds,0), {
                dump_peers_to_file(net.peers, peers_file);
            });
        }
        */

        Ok(net)
    }

    pub async fn wait_event(&self) -> Result<CandidateConnection, Box<dyn Error>> {
        println!("[NetworkController::wait_event] Lauching wait");
        let adr_listener = format!("localhost:{}", self.listen_port);
        let listener = TcpListener::bind(&adr_listener).await?;
        println!("[NetworkController::wait_event] Binded to {}", adr_listener);
        
        let (socket, _) = listener.accept().await?;
        println!("[NetworkController::wait_event] Accepted !");
        Ok(CandidateConnection {
            ip: socket.peer_addr().unwrap().ip().to_string(),
            socket,
            is_outgoing: false
        })
    }

    pub async fn perform_handshake(ip: &String, mut socket: TcpStream, is_outgoing: bool) -> Result<crate::network::controller::HandshakeStatus, Box<dyn Error>> {
        let msg_sent = String::from("Welcome to Massa");
        let msg_rcv = String::from("Thanks !");
        if is_outgoing {
            //let mut stream = TcpStream::connect(&ip).await?;
            //println!("[NetworkController::perform_handshake] Connected !");
            let mut buf = vec![0; 1024];
            let bits_wrote = socket.write(&msg_sent.as_bytes()).await;
            if bits_wrote.unwrap() != msg_sent.as_bytes().len() {
                return Ok(HandshakeStatus::HandshakeFailure);
            }
            println!("[NetworkController::perform_handshake] Wrote <{}> bits!", from_utf8(&buf)?);
            socket.read(&mut buf).await?;
            println!("[NetworkController::perform_handshake] Received response <{}> !", from_utf8(&buf)?);
            if from_utf8(&buf)? == msg_rcv { 
                return Ok(HandshakeStatus::HandshakeSuccess);
            } else {
                return Ok(HandshakeStatus::HandshakeFailure);
            }
        } else {
            let mut buf = vec![0; 1024];

            let sock_read_size = socket.read(&mut buf).await;
            println!("[NetworkController::perform_handshake] Read <{}>", from_utf8(&buf).unwrap());
            if sock_read_size.unwrap() == 0 || from_utf8(&buf)?.eq(msg_sent.as_str()) {
                return Ok(HandshakeStatus::HandshakeFailure);
            }
            println!("[NetworkController::perform_handshake] Writing response ...");

            if socket.write_all(&msg_rcv.as_bytes()).await.is_ok() {
                return Ok(HandshakeStatus::HandshakeSuccess);
            } else {
                return Ok(HandshakeStatus::HandshakeFailure);
            }
        }
    }

    pub async fn feedback_peer_alive(&mut self, ip: &String) -> Result<(), Box<dyn Error>> {
        self.peers.get_mut(ip).unwrap().status = Some(Status::InAlive);
        self.peers.get_mut(ip).unwrap().last_alive = Some(chrono::offset::Utc::now());
        Ok(())
    }

    pub async fn feedback_peer_failed(&mut self, ip: &String) -> Result<(), Box<dyn Error>> {
        self.peers.get_mut(ip).unwrap().status = Some(Status::Idle);
        self.peers.get_mut(ip).unwrap().last_failure = Some(chrono::offset::Utc::now());
        Ok(())
    }

    pub async fn feedback_peer_banned(&mut self, ip: &String) -> Result<(), Box<dyn Error>> {
        self.peers.get_mut(ip).unwrap().status = Some(Status::Banned);
        self.peers.get_mut(ip).unwrap().last_failure = Some(chrono::offset::Utc::now());
        Ok(())
    }

    pub async fn feedback_peer_closed(&mut self, ip: &String) -> Result<(), Box<dyn Error>> {
        self.peers.get_mut(ip).unwrap().status = Some(Status::Idle);
        Ok(())
    }

}
