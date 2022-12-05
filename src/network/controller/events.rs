pub mod NetworkControllerEvent {
    use tokio::net::TcpStream;

    #[derive(Debug, PartialEq)]
    pub enum HandshakeStatus {
        HandshakeSuccess,
        HandshakeFailure
    }
    
    pub struct CandidateConnection {
        pub ip: String,
        pub socket: TcpStream,
        pub is_outgoing: bool
    }}
