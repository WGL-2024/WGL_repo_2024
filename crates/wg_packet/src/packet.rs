use crate::{Query, QueryResult};
use wg_network::{NodeId, SourceRoutingHeader};

// Is atomic unit to be sent
#[derive(Debug, Clone)]
pub struct Packet {
    pub pack_type: PacketType,
    pub routing_header: SourceRoutingHeader,
    pub session_id: u64,
}

#[derive(Debug, Clone)]
pub enum PacketType {
    MsgFragment(Fragment),
    Nack(Nack),
    Ack(Ack),
    Flood(Query),
    FloodResult(QueryResult),
}

#[derive(Debug, Clone)]
pub struct Nack {
    pub fragment_index: u64,
    pub time_of_fail: std::time::Instant,
    pub nack_type: NackType,
}

#[derive(Debug, Clone)]
pub enum NackType {
    ErrorInRouting(NodeId), // contains id of not neighbor
    DestinationIsDrone,
    Dropped,
}

#[derive(Debug, Clone)]
pub struct Ack {
    pub fragment_index: u64,
    pub time_received: std::time::Instant,
}

#[derive(Debug, Clone)]
pub struct Fragment {
    pub fragment_index: u64,
    pub total_n_fragments: u64,
    pub length: u8,
    pub data: [u8; 80],
}
