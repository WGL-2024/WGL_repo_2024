pub type NodeId = u8;

struct SourceRoutingHeader {
	/// Initiator and nodes to which the packet will be forwarded to.
	hops: Vec<NodeID>
}
