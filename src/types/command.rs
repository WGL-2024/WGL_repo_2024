use crate::types::packet::Packet;
use crate::types::source_routing_header::NodeId;
use crossbeam_channel::Sender;

pub enum Command {
    AddChannel(NodeId, Sender<Packet>),
    RemoveChannel(NodeId),
    Crash,
}
