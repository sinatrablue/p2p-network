pub mod tests {
    use std::{error::Error, collections::HashMap};

    use tokio::net::TcpStream;

    use crate::network::controller::{NetworkController, events::NetworkControllerEvent, io_json::export_peers_to_json};

    /**
     * Don't know really what to test about new()
     * Check that some attributes have the value we gave as arg ?
     * Or some init values ?
     */
    #[tokio::test]
    async fn new_controller() -> Result<(), Box<dyn Error>> {
        let map_for_outgoing_connections = HashMap::<String, NetworkController>::new();
        let listen_port = 8080;
        let net = NetworkController::new(
            String::from("peers_list.json"),
            listen_port,
            map_for_outgoing_connections,
            5,
            5,
            5,
            5,
            2,
            10)
        .await?;

        assert_eq!(None, net.status);
        assert_eq!(None, net.last_alive);
        assert_eq!(None, net.last_failure);
        assert_eq!(listen_port, net.listen_port);

        Ok(())
    }

    /**
     * Expect to receive the TcpStream to work with
     * => then attempt to connect to it
     */
    #[tokio::test]
    async fn wait_for_connec() -> Result<(), Box<dyn Error>> {
        let map_for_outgoing_connections = HashMap::<String, NetworkController>::new();
        let net = NetworkController::new(
            String::from("peers_list.json"),
            8080,
            map_for_outgoing_connections,
            5,
            5,
            5,
            5,
            2,
            10)
        .await?;

        let controller_stream = net.wait_event().await?;
        tokio::spawn(async move {
            assert!(TcpStream::connect(format!("{}:{}", controller_stream.ip, 8080)).await.is_ok());
        });

        Ok(())
    }

    /**
     * Try to perform handshake in the good / bad way
     */
    #[tokio::test]
    async fn clean_handshake() -> Result<(), Box<dyn Error>> {
        let map_for_outgoing_connections = HashMap::<String, NetworkController>::new();
        let net = NetworkController::new(
            String::from("peers_list.json"),
            8080,
            map_for_outgoing_connections,
            5,
            5,
            5,
            5,
            2,
            10)
        .await?;

        tokio::select! {
            evt = net.wait_event() => match evt {
                Ok(msg) => match msg {
                    NetworkControllerEvent::CandidateConnection {ip, socket, is_outgoing} => {
                        assert_eq!(NetworkController::perform_handshake(&ip, socket, is_outgoing).await.unwrap(), NetworkControllerEvent::HandshakeStatus::HandshakeSuccess);
                    }
                },
                Err(e) => return Err(e)
            }
        }

        Ok(())
    }
/*
    async fn perform_bad_handshake(ip: i32, socket: TcpStream, is_outgoing: bool) -> Result<(), Box<dyn Error>> {

        Ok(())
    }
    #[tokio::test]
    async fn bad_handshake() -> Result<(), Box<dyn Error>> {
        let map_for_outgoing_connections = HashMap::<String, NetworkController>::new();
        let mut net = NetworkController::new(
            String::from("peers_list.json"),
            8080,
            map_for_outgoing_connections,
            5,
            5,
            5,
            5,
            2,
            10)
        .await?;

        tokio::select! {
            evt = net.wait_event() => match evt {
                Ok(msg) => match msg {
                    NetworkControllerEvent::CandidateConnection {ip, socket, is_outgoing} => {
                        assert_eq!(perform_bad_handshake(ip, socket, is_outgoing), NetworkControllerEvent::HandshakeFailure);
                    }
                },
                Err(e) => return Err(e)
            }
        }
        Ok(())
    }
    */

    /**
     * Expect to find the elements modified as supposed to
     */
    #[tokio::test]
    async fn peer_alive() -> Result<(), Box<dyn Error>> {
        let map_for_outgoing_connections = HashMap::<String, NetworkController>::new();
        let mut net = NetworkController::new(
            String::from("peers_list.json"),
            8080,
            map_for_outgoing_connections,
            5,
            5,
            5,
            5,
            2,
            10)
        .await?;
        
        let ip = String::from("localhost:9876");
        let last_checked_date = Some(chrono::offset::Utc::now());

        net.feedback_peer_alive(&ip).await?;

        assert_eq!(net.peers.get(&ip)
            .unwrap()
            .status.as_ref()
            .unwrap()
            .to_string(), "InAlive");

        assert!(
            net.last_alive > last_checked_date,
            "Dates were => {}  //  {}",
            net.last_alive.unwrap().to_string(),
            last_checked_date.unwrap().to_string()
        );

        Ok(())
    }

    #[tokio::test]
    async fn peer_failed() -> Result<(), Box<dyn Error>> {
        let map_for_outgoing_connections = HashMap::<String, NetworkController>::new();
        let mut net = NetworkController::new(
            String::from("peers_list.json"),
            8080,
            map_for_outgoing_connections,
            5,
            5,
            5,
            5,
            2,
            10)
        .await?;
        
        let ip = String::from("localhost:9876");
        let last_checked_date = Some(chrono::offset::Utc::now());

        net.feedback_peer_failed(&ip).await?;

        assert_eq!(net.peers.get(&ip)
            .unwrap()
            .status.as_ref()
            .unwrap()
            .to_string(), "Idle");
            
        assert!(
            net.last_failure > last_checked_date,
            "Dates were => {}  //  {}",
            net.last_failure.unwrap().to_string(),
            last_checked_date.unwrap().to_string()
        );

        Ok(())
    }

    #[tokio::test]
    async fn peer_banned() -> Result<(), Box<dyn Error>> {
        let map_for_outgoing_connections = HashMap::<String, NetworkController>::new();
        let mut net = NetworkController::new(
            String::from("peers_list.json"),
            8080,
            map_for_outgoing_connections,
            5,
            5,
            5,
            5,
            2,
            10)
        .await?;
        
        let ip = String::from("localhost:9876");
        let last_checked_date = Some(chrono::offset::Utc::now());

        net.feedback_peer_banned(&ip).await?;

        assert_eq!(net.peers.get(&ip)
            .unwrap()
            .status.as_ref()
            .unwrap()
            .to_string(), "Banned");
            
        assert!(
            net.last_failure > last_checked_date,
            "Dates were => {}  //  {}",
            net.last_failure.unwrap().to_string(),
            last_checked_date.unwrap().to_string()
        );

        Ok(())
    }

    #[tokio::test]
    async fn peer_closed() -> Result<(), Box<dyn Error>> {
        let map_for_outgoing_connections = HashMap::<String, NetworkController>::new();
        let mut net = NetworkController::new(
            String::from("peers_list.json"),
            8080,
            map_for_outgoing_connections,
            5,
            5,
            5,
            5,
            2,
            10)
        .await?;
        
        let ip = String::from("localhost:9876");

        net.feedback_peer_closed(&ip).await?;

        assert_eq!(net.peers.get(&ip)
            .unwrap()
            .status.as_ref()
            .unwrap()
            .to_string(), "Idle");
            
        Ok(())
    }

    #[tokio::test]
    async fn json_io_put() -> Result<(), Box<dyn Error>> {
        let map_for_outgoing_connections = HashMap::<String, NetworkController>::new();
        let mut net = NetworkController::new(
            String::from("peers_list.json"),
            8080,
            map_for_outgoing_connections,
            5,
            5,
            5,
            5,
            2,
            10)
        .await?;

        let map_for_outgoing_connections1 = HashMap::<String, NetworkController>::new();
        let net1 = NetworkController::new(
            String::from("peers_list.json"),
            8080,
            map_for_outgoing_connections1,
            5,
            5,
            5,
            5,
            2,
            10)
        .await?;
        
        net.peers.insert(String::from("localhost:8080"), net1);
        export_peers_to_json(net.peers, String::from("peers_list.json"))?;

        Ok(())
    }
    /*
    #[tokio::test]
    async fn sign_of_life() -> Result<(), Box<dyn Error>> {
        let map_for_outgoing_connections = HashMap::<String, NetworkController>::new();
        let mut net = NetworkController::new(
            String::from("peers_list.json"),
            8080,
            map_for_outgoing_connections,
            5,
            5,
            5,
            5,
            2,
            10)
        .await?;

        let mut sign_count = 0;
        
        tokio::time::timeout(tokio::time::Duration::new(5,0), {
            loop {
                tokio::select! {
                    evt = net.wait_event() => match evt {
                        Ok(msg) => match msg {
                            NetworkControllerEvent::CandidateConnection {ip, socket, is_outgoing} => {
                                assert_eq!(perform_handshake(ip, socket, is_outgoing), NetworkEvent::HandshakeSuccess);
                            }
                                    
                            NetworkControllerEvent::PingAlive => {
                                sign_count += 1;
                            }
                        },
                        Err(e) => return Err(e)
                    }
                }
            }
        }).await.unwrap();

        assert!(sign_count > 0);
        Ok(())
    }

    async fn missbehave_to_peer() -> Result<(), Box<dyn Error>> {

        Ok(())
    }

    async fn close_connection() -> Result<(), Box<dyn Error>> {

        Ok(())
    }
    */
}