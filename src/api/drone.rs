use crate::types::command::Command;
use crate::types::source_routing_header::NodeId;
use crossbeam_channel::{Receiver, Sender};
use crate::types::packet::Packet;

// This is a drone of a group
// Pass to it only what it need to know
pub trait Drone{
    fn new(
        id: NodeId,
        sim_contr_send: Sender<Command>,
        sim_contr_recv: Receiver<Command>,
        packet_recv: Receiver<Packet>,
        pdr: f32,
    ) -> Self;
    // Other nodes are added by sending command
    // using the simulation control channel to send
    // Command(AddChannel(...))

    fn run(&mut self);
}
