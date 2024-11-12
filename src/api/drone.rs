use crate::types::command::Command;
use crate::types::packet::Packet;
use crate::types::source_routing_header::NodeId;
use crossbeam_channel::{Receiver, Sender};
use std::collections::HashMap;

//This struct will be used inside the threads created by the initializer.
pub struct Drone {
    pub drone_id: NodeId,
    pub sim_contr_send: Sender<Command>,   //Not packet.
    pub sim_contr_recv: Receiver<Command>, //Not packet.
    pub packet_send: HashMap<NodeId, Sender<Packet>>, //All the sender to other nodes.
    pub packet_recv: Receiver<Packet>, //This drone receiver, that will be linked to a sender given to every other drone.
    pub pdr: u8,                       //Would keep it in % to occupy less space, but could be f32.
}

pub trait DroneImplement {
    fn new_drone(
        id: NodeId,
        scs: Sender<Command>,
        scr: Receiver<Command>,
        ps: HashMap<NodeId, Sender<Packet>>,
        pr: Receiver<Packet>,
        pdr: f32,
    ) -> Drone;
}
