use crate::{FloodRequest, FloodResponse};
use std::fmt::{Debug, Display, Formatter};
use wg_network::{NodeId, SourceRoutingHeader};

pub const FRAGMENT_DSIZE: usize = 128;

// Is atomic unit to be sent
#[derive(Debug, Clone)]
#[cfg_attr(feature = "debug", derive(PartialEq))]
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
    pub fn get_fragment_index(&self) -> u64 {
        match &self.pack_type {
            PacketType::MsgFragment(fragment) => fragment.fragment_index,
            _ => 0,
        }
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Packet({}) {{ routing_header: {}, pack_type {} }}",
            self.session_id, self.routing_header, self.pack_type
        )
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "debug", derive(PartialEq))]
pub enum PacketType {
    MsgFragment(Fragment),
    Ack(Ack),
    Nack(Nack),
    FloodRequest(FloodRequest),
    FloodResponse(FloodResponse),
}

impl Display for PacketType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketType::MsgFragment(fragment) => write!(f, "{}", fragment),
            PacketType::Ack(ack) => write!(f, "{}", ack),
            PacketType::Nack(nack) => write!(f, "{}", nack),
            PacketType::FloodRequest(flood_request) => write!(f, "{}", flood_request),
            PacketType::FloodResponse(flood_response) => write!(f, "{}", flood_response),
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "debug", derive(PartialEq))]
pub struct Nack {
    pub fragment_index: u64, // If the packet is not a fragment, it's considered as a whole, so fragment_index will be 0.
    pub nack_type: NackType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NackType {
    ErrorInRouting(NodeId), // contains id of not neighbor
    DestinationIsDrone,
    Dropped,
    UnexpectedRecipient(NodeId),
}

impl Display for Nack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "debug", derive(PartialEq))]
pub struct Ack {
    pub fragment_index: u64,
}

impl Display for Ack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ack({})", self.fragment_index)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "debug", derive(PartialEq))]
pub struct Fragment {
    pub fragment_index: u64,
    pub total_n_fragments: u64,
    pub length: u8,
    pub data: [u8; FRAGMENT_DSIZE],
}

/// This prints something like this:
/// Fragment { index: 1/2, data: 0xf219a352ddfc1b4a... + other 60 bytes }
impl Display for Fragment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.length < 20 {
            write!(
                f,
                "Fragment {{ index: {} out of {}, data: 0x{} }}",
                self.fragment_index + 1,
                self.total_n_fragments,
                self.data
                    .iter()
                    .take(self.length as usize)
                    .fold(String::new(), |acc, b| format!("{acc}{b:02x}"))
            )
        } else {
            write!(
                f,
                "Fragment {{ index: {} out of {}, data: 0x{}... + other {} bytes }}",
                self.fragment_index + 1,
                self.total_n_fragments,
                self.data
                    .iter()
                    .take(20)
                    .fold(String::new(), |acc, b| format!("{acc}{b:02x}")),
                self.length - 20
            )
        }
    }
}

impl Fragment {
    pub fn new(fragment_index: u64, total_n_fragments: u64, data: [u8; FRAGMENT_DSIZE]) -> Self {
        let length = data.iter().position(|&b| b == 0).unwrap_or(FRAGMENT_DSIZE) as u8;
        Self {
            fragment_index,
            total_n_fragments,
            length,
            data,
        }
    }
    pub fn from_string(fragment_index: u64, total_n_fragments: u64, raw_data: String) -> Self {
        let mut data = [0; FRAGMENT_DSIZE];
        let length = raw_data.len().min(FRAGMENT_DSIZE);
        data[..length].copy_from_slice(raw_data.as_bytes());
        Self {
            fragment_index,
            total_n_fragments,
            length: length as u8,
            data,
        }
    }
}
