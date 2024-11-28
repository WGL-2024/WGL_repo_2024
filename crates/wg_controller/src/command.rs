use crossbeam_channel::Sender;
use wg_network::NodeId;
use wg_packet::Packet;

/// From controller to drone
#[derive(Debug, Clone)]
pub enum DroneCommand {
    AddSender(NodeId, Sender<Packet>),
    SetPacketDropRate(f32),
    Crash,
}

// we consider two AddSender commands to be equal if they
// tell a drone to add a Sender for the same neighbor id
impl PartialEq for DroneCommand {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DroneCommand::AddSender(node1, _), DroneCommand::AddSender(node2, _)) => {
                node1 == node2
            }
            (DroneCommand::SetPacketDropRate(rate1), DroneCommand::SetPacketDropRate(rate2)) => {
                rate1 == rate2
            }
            (DroneCommand::Crash, DroneCommand::Crash) => true,
            _ => false,
        }
    }
}

/// From drone to controller
#[derive(Debug, Clone, PartialEq)]
pub enum NodeEvent {
    PacketSent(Packet),
    PacketDropped(Packet),
}
