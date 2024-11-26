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
    pub fn increment(&self, node_id: NodeId, node_type: NodeType) -> FloodRequest {
        let mut path_trace = self.path_trace.clone();
        path_trace.push((node_id, node_type));
        FloodRequest {
            flood_id: self.flood_id,
            initiator_id: self.initiator_id,
            path_trace,
        }
    }
    pub fn generate_response(&self) -> FloodResponse {
        let mut path_trace = self.path_trace.clone();
        path_trace.reverse();
        FloodResponse {
            flood_id: self.flood_id,
            path_trace,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FloodResponse {
    pub flood_id: u64,
    pub path_trace: Vec<(NodeId, NodeType)>,
}
