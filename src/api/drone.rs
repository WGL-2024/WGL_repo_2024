use crate::types::command::Command;
use crate::types::packet::Packet;
use crate::types::source_routing_header::NodeId;
use crossbeam_channel::{Receiver, Sender};

// This is a drone of a group
// Pass to it only what it need to know
pub trait Drone {
    fn new(
        id: NodeId,
        sim_contr_send: Sender<Command>,
        sim_contr_recv: Receiver<Command>,
        packet_recv: Receiver<Packet>,
        pdr: f32,
    ) -> Self;
    // The list packet_send would be crated empty inside new.
    // Other nodes are added by sending command
    // using the simulation control channel to send 'Command(AddChannel(...))'.

    fn run(&mut self);
}
