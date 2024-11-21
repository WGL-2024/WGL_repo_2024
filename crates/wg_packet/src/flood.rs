use wg_network::{NodeId, SourceRoutingHeader};

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
    pub ttl: u8,
    pub path_trace: Vec<(NodeId, NodeType)>,
}

#[derive(Debug, Clone)]
pub struct FloodResponse {
    pub flood_id: u64,
    pub source_routing_header: SourceRoutingHeader,
    pub path_trace: Vec<(NodeId, NodeType)>,
}
