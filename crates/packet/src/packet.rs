use network::{NodeId, SourceRoutingHeader};

// Is atomic unit to be sent
pub struct Packet {
    pub pack_type: PacketType,
    pub routing_header: SourceRoutingHeader,
    pub session_id: u64,
}

pub enum PacketType {
    MsgFragment(Fragment),
    Nack(Nack),
    Ack(Ack),
}

pub struct Nack {
    pub fragment_index: u64,
    pub time_of_fail: std::time::Instant,
    pub nack_type: NackType,
}

pub enum NackType {
    ErrorInRouting(NodeId), // contains id of not neighbor
    Dropped(),
}

pub struct Ack {
    pub fragment_index: u64,
    pub time_received: std::time::Instant,
}

pub struct Fragment {
    pub fragment_index: u64,
    pub total_n_fragments: u64,
    pub data: FragmentData,
}

pub struct FragmentData {
    pub length: u8,
    pub data: [u8; 80],
}
