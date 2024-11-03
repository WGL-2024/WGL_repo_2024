// Should be u8
pub type NodeId = u64;

// False type to make cargo run happy
pub type SourceRoutingHeader = [NodeId; 16];

pub mod message;
pub mod packet;
