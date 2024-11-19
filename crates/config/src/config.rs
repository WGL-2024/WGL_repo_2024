use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Drone {
    pub id: u64,
    pub connected_drone_ids: Vec<u64>,
    pub pdr: f64,
}

#[derive(Debug, Deserialize)]
pub struct Client {
    pub id: u64,
    pub connected_drone_ids: Vec<u64>,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub id: u64,
    pub connected_drone_ids: Vec<u64>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub drone: Vec<Drone>,
    pub client: Vec<Client>,
    pub server: Vec<Server>,
}
