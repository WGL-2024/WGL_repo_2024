use crossbeam_channel::Sender;
use wg_network::NodeId;
use wg_packet::Packet;

#[derive(Debug, Clone)]
pub enum Command {
    AddChannel(NodeId, Sender<Packet>),
    RemoveChannel(NodeId),
    Crash,
}
