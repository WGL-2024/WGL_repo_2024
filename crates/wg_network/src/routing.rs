pub type NodeId = u8;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "partial_eq", derive(PartialEq))]
pub struct SourceRoutingHeader {
    pub hop_index: usize, // must be set to 1 initially by the sender
    // Initiator and nodes to which the packet will be forwarded to.
    pub hops: Vec<NodeId>,
}
