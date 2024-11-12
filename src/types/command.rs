use crate::types::packet::Packet;
use crate::types::source_routing_header::NodeId;
use crossbeam_channel::Sender;

pub enum Command {
    ADDCHANNEL(NodeId, Sender<Packet>),
    REMOVECHANNEL(NodeId),
    CRASH,
}
