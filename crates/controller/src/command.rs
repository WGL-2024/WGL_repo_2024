use crossbeam_channel::Sender;
use network::NodeId;
use packet::Packet;

pub enum Command {
    AddChannel(NodeId, Sender<Packet>),
    RemoveChannel(NodeId),
    Crash,
}
