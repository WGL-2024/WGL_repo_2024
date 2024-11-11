/*

this file showcases an example of how you can parse the config found in the Network Initialization File inside some structs, which can then be used to initialize the network

remember to add the Dependencies for serde and toml to Cargo.toml:

[dependencies]
toml = "0.8.19"
serde = { version = "1.0.214", features = ["derive"] }

*/

use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Drone {
    id: u64,
    connected_drone_ids: Vec<u64>,
    pdr: f64,
}

#[derive(Debug, Deserialize)]
struct Client {
    id: u64,
    connected_drone_ids: Vec<u64>,
}

#[derive(Debug, Deserialize)]
struct Server {
    id: u64,
    connected_drone_ids: Vec<u64>,
}

#[derive(Debug, Deserialize)]
struct Config {
    drone: Vec<Drone>,
    client: Vec<Client>,
    server: Vec<Server>,
}

fn main() {
    let config_data = fs::read_to_string("src/config.toml").expect("Unable to read config file");
    // having our structs implement the Deserialize trait allows us to use the toml::from:str function to deserialize the config file into each of them
    let config: Config = toml::from_str(&config_data).expect("Unable to parse TOML");
    println!("{:#?}", config);
}
