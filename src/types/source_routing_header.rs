pub type NodeId = u8;

// False types to make cargo run happy
pub type SourceRoutingHeader = [NodeId; 16];
