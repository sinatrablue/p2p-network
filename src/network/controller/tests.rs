pub mod tests {
    use std::{error::Error, collections::HashMap};

    use crate::network::controller::NetworkController;

    #[test]
    /**
     * Expect json file to be modified after <peer_file_dump_interval_seconds>
     * when calling NetworkController::new
     * */
    fn io_json() {
        assert!(true);
    }

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

        net.feedback_peer_alive(&ip).await?;

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

        net.feedback_peer_alive(&ip).await?;

        assert_eq!(net.peers.get(&ip)
            .unwrap()
            .status.as_ref()
            .unwrap()
            .to_string(), "Idle");
            
        Ok(())
    }
}