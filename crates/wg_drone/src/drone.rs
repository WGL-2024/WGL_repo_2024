use std::collections::HashMap;

use crossbeam_channel::{Receiver, Sender};
use wg_controller::{DroneCommand, NodeEvent};
use wg_network::NodeId;
use wg_packet::Packet;

/// This is the drone interface.
/// Each drone's group must implement it
pub trait Drone {
    /// The list packet_send would be crated empty inside new.
    /// Other nodes are added by sending command
    /// using the simulation control channel to send 'Command(AddChannel(...))'.
    fn new(
        id: NodeId,
        controller_send: Sender<NodeEvent>,
        controller_recv: Receiver<DroneCommand>,
        packet_recv: Receiver<Packet>,
        packet_send: HashMap<NodeId, Sender<Packet>>,
        pdr: f32,
    ) -> Self;

    fn run(&mut self);
}
