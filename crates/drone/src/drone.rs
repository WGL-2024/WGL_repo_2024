use controller::Command;
use crossbeam_channel::{Receiver, Sender};
use network::NodeId;
use packet::Packet;

pub struct DroneOptions {
    pub id: NodeId,
    pub sim_contr_send: Sender<Command>,
    pub sim_contr_recv: Receiver<Command>,
    pub packet_recv: Receiver<Packet>,
    pub pdr: f32,
}

/// This is the drone interface.
/// Each drone's group must implement it
pub trait Drone {
    /// The list packet_send would be crated empty inside new.
    /// Other nodes are added by sending command
    /// using the simulation control channel to send 'Command(AddChannel(...))'.
    fn new(options: DroneOptions) -> Self;

    fn run(&mut self);
}
