#[cfg(feature = "serialize")]
use serde::Deserialize;
use wg_network::NodeId;

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Deserialize))]
pub struct Drone {
    pub id: NodeId,
    pub connected_drone_ids: Vec<NodeId>,
    pub pdr: f32,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Deserialize))]
pub struct Client {
    pub id: NodeId,
    pub connected_drone_ids: Vec<NodeId>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Deserialize))]
pub struct Server {
    pub id: NodeId,
    pub connected_drone_ids: Vec<NodeId>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Deserialize))]
pub struct Config {
    pub drone: Vec<Drone>,
    pub client: Vec<Client>,
    pub server: Vec<Server>,
}
