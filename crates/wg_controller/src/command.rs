use crossbeam_channel::Sender;
use wg_network::NodeId;
use wg_packet::Packet;

pub enum Command {
    AddSender(Sender<Packet>, dst: NodeId),
    RemoveChannel(NodeId),
    Crash,
    SetPacketDropRate(f32),
    //These are messages sent back to the sim controller.
    //The channel uses the same enum.
    Topology(NodeId, nghb: Vec<NodeId>/*, metadata*/),
    MessageSent(src: NodeId, trg: NodeId/*, metadata*/)
}
