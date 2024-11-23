use std::{collections::HashSet, ops::Add};

use wg_network::{NodeId, SourceRoutingHeader};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum NodeType {
    Client,
    Drone,
    Server,
}

#[derive(Debug, Clone)]
pub struct FloodRequest {
    pub flood_id: u64,
    pub prev_hop: NodeId,
    pub to: Option<NodeId>,
    pub reason: FloodReason

}
impl Default for FloodRequest {
    fn default() -> Self {
        FloodRequest{
            flood_id: 0,
            prev_hop: 0,
            to: None,
            reason: FloodReason::Standard,
        }
    }
    
}

#[derive(Debug, Clone)]
pub struct FloodProcessing {
    flood_id: u64,
    processing_node_id: NodeId
}

#[derive(Debug, Clone)]
pub struct FloodData {
    pub flood_id: u64,
    pub from: NodeId,
    pub node_list: HashSet<(NodeId, NodeType)>,
    pub connections: HashSet<(NodeId, NodeId)>, // The first element of the tuple MUST be the one with the lower NodeId, ex. (3,4) is allowed, (4,3) is not.
    pub error_at: Option<NodeId>,
    pub is_broadcast: bool 
}
impl Default for FloodData {
    fn default() -> Self {
        FloodData{
            flood_id: 0,
            from: 0,
            node_list: HashSet::new(),
            connections: HashSet::new(),
            error_at: None,
            is_broadcast: false,
        }
    }

}

impl Add for FloodData {
    type Output = FloodData;
    fn add(self, rhs: Self) -> Self::Output {
        FloodData{
            flood_id: self.flood_id,
            from: 0,
            node_list: self.node_list.union(&rhs.node_list).copied().collect(),
            connections: self.connections.union(&rhs.connections).copied().collect(),
            error_at: self.error_at.or(rhs.error_at),
            is_broadcast: self.is_broadcast || rhs.is_broadcast,
        }
    }
}


#[derive(Debug,Clone)]
pub enum FloodReason {
    Standard,
    PreviousFloodFailed,
}