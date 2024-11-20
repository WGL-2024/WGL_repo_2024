pub type NodeId = u8;

#[derive(Debug)]
pub struct SourceRoutingHeader {
    pub hop_index: usize, // must be set to 0 initially by the sender
    // Initiator and nodes to which the packet will be forwarded to.
    pub hops: Vec<NodeId>,
}
