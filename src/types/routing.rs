use crate::types::node::NodeType;
use std::rc::Rc;
use std::sync::Arc;

type NodeId = u64;

#[derive(Debug)]
pub struct SourceRoutingHeader {
    /// List of nodes to which to forward the packet.
    pub hops: Vec<NodeId>,
    /// Index of the receiving node in the hops field.
    /// Ranges from 0 to n_hops - 1.
    pub hop_index: u64,
}
pub struct Query {
    /// Unique identifier of the flood, to prevent loops.
    flood_id: u64,
    /// ID of client or server
    initiator_id: Rc<Arc<usize>>,
    /// Time To Live, decremented at each hop to limit the query's lifespan.
    ttl: u8,
    /// Records the nodes that have been traversed (to track the connections).
    path_trace: [u64; 20],
    node_types: Vec<NodeType>,
}
