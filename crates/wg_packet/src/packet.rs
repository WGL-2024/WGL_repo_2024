use wg_network::{topology::NodeType, NodeId, SourceRoutingHeader};

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
	Query(Query),
	QueryResult(QueryResult),
}

pub struct Nack {
    pub fragment_index: u64,
    pub time_of_fail: std::time::Instant,
    pub nack_type: NackType,
}

pub enum NackType {
    ErrorInRouting(NodeId), // contains id of not neighbor
    DestinationIsDrone,
    Dropped,
}

pub struct Ack {
    pub fragment_index: u64,
    pub time_received: std::time::Instant,
}

pub struct Query {
	/// Unique identifier of the flood, to prevent loops.
	flood_id: u64,
	/// ID of client or server
	initiator_id: NodeId,
	/// Time To Live, decremented at each hop to limit the query's lifespan.
	/// When ttl reaches 0, we start a QueryResult message that reaches back to the initiator
	ttl: u8,
	/// Records the nodes that have been traversed (to track the connections).
	path_trace: Vec<(u64, NodeType)>
}

pub struct QueryResult {
	flood_id: u64,
	source_routing_header: SourceRoutingHeader,
	path_trace: Vec<(u64, NodeType)>
}

pub struct Fragment {
    pub fragment_index: u64,
    pub total_n_fragments: u64,
    pub length: u8,
    pub data: [u8; 80],
}