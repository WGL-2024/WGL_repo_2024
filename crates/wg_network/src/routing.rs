pub type NodeId = u8;

#[derive(Debug)]
pub struct SourceRoutingHeader {
    /// Initiator and nodes to which the packet will be forwarded to.
    hops: Vec<NodeId>,
}
