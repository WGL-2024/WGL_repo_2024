use crossbeam_channel::Sender;
use wg_network::NodeId;
use wg_packet::Packet;

/// From controller to drone
#[derive(Debug, Clone)]
pub enum DroneCommand {
    AddNeighbor(NodeId, Sender<Packet>),
    RemoveNeighbor(NodeId),
    SetPacketDropRate(f32),
    Crash,
}

/// From drone to controller
#[derive(Debug, Clone)]
pub enum ControllerCommand {
    PacketSent(Packet),
    PacketDropped(Packet),
}
