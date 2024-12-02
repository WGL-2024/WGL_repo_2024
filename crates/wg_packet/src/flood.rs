use std::fmt::Display;
use wg_network::NodeId;

#[derive(Debug, Clone)]
pub enum NodeType {
    Client,
    Drone,
    Server,
}

#[derive(Debug, Clone)]
pub struct FloodRequest {
    pub flood_id: u64,
    pub initiator_id: NodeId,
    pub path_trace: Vec<(NodeId, NodeType)>,
}

impl FloodRequest {
    pub fn new(flood_id: u64, initiator_id: NodeId) -> Self {
        Self {
            flood_id,
            initiator_id,
            path_trace: Vec::new(),
        }
    }
    pub fn initialize(flood_id: u64, initiator_id: NodeId, initiator_type: NodeType) -> Self {
        Self {
            flood_id,
            initiator_id,
            path_trace: vec![(initiator_id, initiator_type)],
        }
    }
    pub fn increment(&mut self, node_id: NodeId, node_type: NodeType) {
        self.path_trace.push((node_id, node_type));
    }
    pub fn get_incremented(&self, node_id: NodeId, node_type: NodeType) -> FloodRequest {
        let mut clone = self.clone();
        clone.increment(node_id, node_type);
        clone
    }
    pub fn generate_response_reversed(&self) -> FloodResponse {
        let mut path_trace = self.path_trace.clone();
        path_trace.reverse();
        FloodResponse {
            flood_id: self.flood_id,
            path_trace,
        }
    }
    pub fn generate_response(&self) -> FloodResponse {
        FloodResponse {
            flood_id: self.flood_id,
            path_trace: self.path_trace.clone(),
        }
    }
}

impl Display for FloodRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct FloodResponse {
    pub flood_id: u64,
    pub path_trace: Vec<(NodeId, NodeType)>,
}

impl Display for FloodResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
