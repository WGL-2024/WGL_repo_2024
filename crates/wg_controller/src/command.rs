use crossbeam_channel::Sender;
use wg_network::NodeId;
use wg_packet::Packet;

/// From controller to drone
#[derive(Debug, Clone)]
pub enum DroneCommand {
    RemoveSender(NodeId),
    AddSender(NodeId, Sender<Packet>),
    SetPacketDropRate(f32),
    Crash,
}

#[cfg(feature = "debug")]
impl PartialEq for DroneCommand {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DroneCommand::RemoveSender(node1), DroneCommand::RemoveSender(node2)) => {
                node1 == node2
            }
            (DroneCommand::AddSender(node1, sender1), DroneCommand::AddSender(node2, sender2)) => {
                node1 == node2 && sender1.same_channel(sender2)
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
#[derive(Debug, Clone)]
#[cfg_attr(feature = "debug", derive(PartialEq))]
pub enum DroneEvent {
    PacketSent(Packet),         // drone sent a packet
    PacketReceived(Packet),     // drone received a packet
    PacketDropped(Packet),      // drone dropped this packet
    DroneCrash, // the drone has managed any packets remaining in its queues and is about to get dropped
    ControllerShortcut(Packet), // Used for direct routing of Ack, Nack and FloodResponse
}
