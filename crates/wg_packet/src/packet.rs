use crate::{FloodRequest, FloodResponse};
use std::fmt::{Debug, Formatter};
use wg_network::{NodeId, SourceRoutingHeader};

// Is atomic unit to be sent
#[derive(Debug, Clone)]
pub struct Packet {
    pub routing_header: SourceRoutingHeader,
    pub session_id: u64,
    pub pack_type: PacketType,
}

impl Packet {
    pub fn new_fragment(
        routing_header: SourceRoutingHeader,
        session_id: u64,
        fragment: Fragment,
    ) -> Self {
        Self {
            pack_type: PacketType::MsgFragment(fragment),
            routing_header,
            session_id,
        }
    }
    pub fn new_ack(
        routing_header: SourceRoutingHeader,
        session_id: u64,
        fragment_index: u64,
    ) -> Self {
        Self {
            pack_type: PacketType::Ack(Ack { fragment_index }),
            routing_header,
            session_id,
        }
    }
    pub fn new_nack(routing_header: SourceRoutingHeader, session_id: u64, nack: Nack) -> Self {
        Self {
            pack_type: PacketType::Nack(nack),
            routing_header,
            session_id,
        }
    }
    pub fn new_flood_request(
        routing_header: SourceRoutingHeader,
        session_id: u64,
        flood_request: FloodRequest,
    ) -> Self {
        Self {
            pack_type: PacketType::FloodRequest(flood_request),
            routing_header,
            session_id,
        }
    }
    pub fn new_flood_response(
        routing_header: SourceRoutingHeader,
        session_id: u64,
        flood_response: FloodResponse,
    ) -> Self {
        Self {
            pack_type: PacketType::FloodResponse(flood_response),
            routing_header,
            session_id,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PacketType {
    MsgFragment(Fragment),
    Ack(Ack),
    Nack(Nack),
    FloodRequest(FloodRequest),
    FloodResponse(FloodResponse),
}

#[derive(Debug, Clone)]
pub enum Nack {
    ErrorInRouting(NodeId), // contains id of not neighbor
    DestinationIsDrone,
    Dropped(u64), // fragment id of dropped fragment
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

/// This prints something like this:
/// Fragment { index: 1/2, data: 0xf219a352ddfc1b4a... + other 60 bytes }
impl Debug for Fragment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.length < 20 {
            write!(
                f,
                "Fragment {{ index: {}/{}, data: 0x{} }}",
                self.fragment_index + 1,
                self.total_n_fragments,
                self.data
                    .iter()
                    .take(self.length as usize)
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        } else {
            write!(
                f,
                "Fragment {{ index: {}/{}, data: 0x{}... + other {} bytes }}",
                self.fragment_index + 1,
                self.total_n_fragments,
                self.data
                    .iter()
                    .take(20)
                    .fold(String::new(), |acc, b| acc + &format!("{:02x}", b)),
                self.length - 20
            )
        }
    }
}
