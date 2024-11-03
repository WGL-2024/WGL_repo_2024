use crate::types::NodeId;
use crate::types::SourceRoutingHeader;

// Is atomic unit to be sent
pub struct Packet {
    pack_type: PacketType,
    routing_header: SourceRoutingHeader,
    session_id: u64,
}

enum PacketType {
    Message(Fragment),
    Nack(Nack),
    Ack(Ack),
}

pub enum Nack {
    ErrorInRouting(NodeId), // contains id of not neighbor
    Dropped,
}

pub struct Ack {
    received_time: std::time::Instant,
}

pub struct Fragment {
    fragment_index: u64,
    total_n_fragments: u64,
    data: FragmentData,
}

struct FragmentData {
    length: u8,
    data: [u8; 80],
}
