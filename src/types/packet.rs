use crate::types::source_routing_header::{NodeId, SourceRoutingHeader};

// Is atomic unit to be sent
pub struct Packet {
    pack_type: PacketType,
    routing_header: SourceRoutingHeader,
    session_id: u64,
}

pub enum PacketType {
    MsgFragment(Fragment),
    Nack(Nack),
    Ack(Ack),
}

pub struct Nack {
    fragment_index: u64,
    time_of_fail: std::time::Instant,
    nack_type: NackType,
}

pub enum NackType {
    ErrorInRouting(NodeId), // contains id of not neighbor
    Dropped(),
}

pub struct Ack {
    fragment_index: u64,
    time_received: std::time::Instant,
}

pub struct Fragment {
    fragment_index: u64,
    total_n_fragments: u64,
    data: FragmentData,
}

pub struct FragmentData {
    length: u8,
    data: [u8; 80],
}
