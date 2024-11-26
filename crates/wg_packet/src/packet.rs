use std::fmt::{Debug, Formatter};
use crate::{FloodRequest, FloodResponse};
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
    FloodRequest(FloodRequest),
    FloodResponse(FloodResponse),
}

#[derive(Debug, Clone)]
pub struct Nack {
    pub fragment_index: u64,
    pub nack_type: NackType,
}

#[derive(Debug, Clone)]
pub enum NackType {
    ErrorInRouting(NodeId), // contains id of not neighbor
    DestinationIsDrone,
    Dropped,
    UnexpectedRecipient(NodeId),
}

#[derive(Debug, Clone)]
pub struct Ack {
    pub fragment_index: u64,
}

#[derive(Clone)]
pub struct Fragment {
    pub fragment_index: u64,
    pub total_n_fragments: u64,
    pub length: u8,
    pub data: [u8; 80],
}

impl Debug for Fragment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.length < 20 {
            write!(f, "Fragment {{ index: {}/{}, data: 0x{} }}", self.fragment_index, self.total_n_fragments, self.data.iter().take(self.length as usize).map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" "))
        } else {
            write!(f, "Fragment {{ index: {}/{}, data: 0x{}... + other {} bytes }}", self.fragment_index, self.total_n_fragments, self.data.iter().take(20).map(|b| format!("{:02x}", b)).collect::<String>(), self.length - 20)
        }
    }
}
