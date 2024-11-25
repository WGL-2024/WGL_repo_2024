use crossbeam_channel::{Receiver, Sender};
use std::collections::HashMap;
use wg_controller::Command;
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
        sim_contr_send: Sender<Command>,
        sim_contr_recv: Receiver<Command>,
        packet_send: HashMap<NodeId, Sender<Packet>>,
        packet_recv: Receiver<Packet>,
        pdr: f32,
    ) -> Self;

    fn run(&mut self);
}
