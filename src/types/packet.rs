use crate::types::routing::SourceRoutingHeader;
type NodeId = u64;

pub struct Packet {
    //fragment defined as entity exchanged by the drones.
    pt: PacketType,
    source_routing_header: SourceRoutingHeader,
    session_id: u64, //sourcerouting header is inverted if necessary.
}

pub struct Fragment {
    // fragment defined as part of a message.
    header: FragmentHeader,
    data: FragmentData,
}
enum PacketType {
    MsgPack(Fragment),
    NackPack(Nack),
    AckPack(Ack),
}
struct FragmentData {
    data: [u8; 80], //it's possible to use .into_bytes() so that images
    //can also be encoded->[u8, 80]
    length: u8, // assembler will fragment/defragment data into bytes.
}
pub struct FragmentHeader {
    /// Identifies the session to which this fragment belongs.
    session_id: u64,
    /// Total number of fragments, must be equal or greater than 1.
    total_n_fragments: u64,
    /// Index of the packet, from 0 up to total_n_fragments - 1.
    fragment_index: u64,
}
struct Error {
    session_id: u64,
    id_not_neighbor: String,
    ttl: u8,
}
struct Nack {
    error_type: NackType,
    session_id: u64,
    source_routing_header: SourceRoutingHeader,
    ttl: u64,
}
enum NackType {
    ErrorInRouting(NodeId), // contains id of not neighbor
    Dropped,
}
pub struct Ack {
    source_routing_header: SourceRoutingHeader,
    received_time: std::time::Instant,
}
