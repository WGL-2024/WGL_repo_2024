use crossbeam_channel::{Receiver, Sender};
use wg_controller::{ControllerCommand, DroneCommand};
use wg_network::NodeId;
use wg_packet::Packet;

#[derive(Debug, Clone)]
pub struct DroneOptions {
    pub id: NodeId,
    pub controller_send: Sender<ControllerCommand>,
    pub controller_recv: Receiver<DroneCommand>,
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
