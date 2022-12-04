use tokio::net::TcpStream;

pub enum HanshakeStatus {
    HandshakeSuccess,
    HandshakeFailure
}

pub struct CandidateConnection {
    pub ip: String,
    pub socket: TcpStream,
    pub is_outgoing: bool
}