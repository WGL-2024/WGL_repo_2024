use crossbeam_channel::Sender;
use wg_network::NodeId;
use wg_packet::Packet;

pub enum Command {
    AddSender(Sender<Packet>, NodeId),
    RemoveChannel(NodeId),
    Crash,
    SetPacketDropRate(NodeId, f32),
}
