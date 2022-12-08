use std::{collections::HashMap, error::Error, str::FromStr};
use crate::network::controller::{NetworkController, Status};
use chrono::{DateTime, Utc};
use serde_json::{self, Value};

pub fn import_peers_from_json(peers_file: &String) -> Result<HashMap::<String, NetworkController>, Box<dyn Error>> {
    let mut computed_map = HashMap::<String, NetworkController>::new();

    let peers: HashMap<String, Value> = serde_json::from_str(
        std::fs::read_to_string(&peers_file)
        .unwrap()
        .as_str())?;
    println!("[io_json::import_peers_from_json] Initial peers list : \n{}", serde_json::to_string_pretty(&peers).unwrap());
    
    let keys = peers.keys();
    for key in keys {
        let empty_hash_map = HashMap::<String, NetworkController>::new();
        let v = peers.get(key).unwrap();
        let nc = NetworkController {
            status: Some(Status::from_str(v.get("status").unwrap().as_str().unwrap()).unwrap()),
            last_alive: Some(DateTime::<Utc>::from_str(v.get("last_alive").unwrap().as_str().unwrap()).unwrap()),
            last_failure: Some(DateTime::<Utc>::from_str(v.get("last_failure").unwrap().as_str().unwrap()).unwrap()),
            listen_port: v.get("listen_port").unwrap().as_u64().unwrap() as u16,
            max_incoming_connections: v.get("max_incoming_connections").unwrap().as_u64().unwrap() as u32,
            max_simultaneous_outgoing_connection_attempts: v.get("max_simultaneous_outgoing_connection_attempts").unwrap().as_u64().unwrap() as u32,
            max_simultaneous_incoming_connection_attempts: v.get("max_simultaneous_incoming_connection_attempts").unwrap().as_u64().unwrap() as u32,
            max_idle_peers: v.get("max_idle_peers").unwrap().as_u64().unwrap() as u32,
            max_banned_peers: v.get("max_banned_peers").unwrap().as_u64().unwrap() as u32,
            peers: empty_hash_map,
        };
        computed_map.insert(key.to_owned(), nc);
    }
    Ok(computed_map)
}

pub fn export_peers_to_json(peers: HashMap::<String, NetworkController>, peers_file: String) -> Result<(), Box<dyn Error>> {
    let keys = peers.keys();

    let mut json_string = String::from("{");
    for key in keys {
        println!("key is => {}", &key);
        let insert = format!(
            r#"\"{}\": {{
                \"status\": {},
                \"last_alive\": {},
                \"last_failure\": {},
                \"listen_port\": {},
                \”max_incoming_connections\": {},
                \"max_simultaneous_outgoing_connection_attempts\": {},
                \"max_simultaneous_incoming_connection_attempts\": {},
                \"max_idle_peers\": {},
                \"max_banned_peers\": {}
            }}"#,
            &*key,
            peers.get(&*key).unwrap().status.as_ref().unwrap_or(&Status::Idle).to_string(),
            peers.get(&*key).unwrap().last_alive.unwrap().to_string(),
            peers.get(&*key).unwrap().last_failure.unwrap().to_string(),
            peers.get(&*key).unwrap().listen_port,
            peers.get(&*key).unwrap().max_incoming_connections,
            peers.get(&*key).unwrap().max_simultaneous_outgoing_connection_attempts,
            peers.get(&*key).unwrap().max_simultaneous_incoming_connection_attempts,
            peers.get(&*key).unwrap().max_idle_peers,
            peers.get(&*key).unwrap().max_banned_peers
        );
        println!("insert is => {}", &insert);
        json_string.push_str(insert.as_str());
    }
    json_string.push_str("}");
    println!("json_string is => {}", json_string);

    std::fs::write(
        peers_file,
        serde_json::to_string_pretty(&json_string).unwrap(),
    )?;

    Ok(())
}