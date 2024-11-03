// Should be u8
pub type NodeId = u64;
pub type SourceRoutingHeader = [NodeId; 16];

pub mod message;