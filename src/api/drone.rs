use std::collections::HashMap;
use crate::types::source_routing_header::NodeId;
use crossbeam_channel::{Receiver, Sender};
use crate::types::packet::Packet;

//This struct will be used inside the threads created by the initializer.
pub struct Drone {
    pub drone_id: NodeId,
    pub sim_contr_send: Sender<Packet>, //Not packet.
    pub sim_contr_recv: Receiver<Packet>, //Not packet.
    pub packet_send: HashMap<NodeId, Sender<Packet>>, //All the sender to other nodes.
    pub packet_recv: Receiver<Packet>, //This drone receiver, that will be linked to a sender given to every other drone.
    pub pdr: u8, //Would keep it in % to occupy less space, but could be f32.
}

impl Drone {
    pub fn new(id: NodeId, scs: Sender<Packet>, scr: Receiver<Packet>, ps: HashMap<NodeId, Sender<Packet>>, pr: Receiver<Packet>, pdr: f32) -> Drone {
        Drone {
            drone_id: id,
            sim_contr_send: scs,
            sim_contr_recv: scr,
            packet_send: ps,
            packet_recv: pr,
            pdr: (pdr*100.0) as u8,
        }
    }
}

pub trait DroneImplement {
    fn run(&mut self) {}
}

