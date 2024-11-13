//THIS IS JUST AN EXAMPLE OF IMPLEMENTATION

use crate::api::drone::Drone;
use crate::types::command::Command;
use crate::types::packet::{Packet, PacketType};
use crate::types::source_routing_header::NodeId;
use crossbeam_channel::{select, Receiver, Sender};
use std::collections::HashMap;
use std::thread;

fn main() {
    // Something like this will be done
    // by the initialization controller
    let handler = thread::spawn(move || {
        let id = 0;
        let drone = Drone::new(id /*require  other parameter here not given*/);

        drone.run();
    });
}

// Example of drone implementation
struct MyDrone {
    id: NodeId,
    sim_contr_send: Sender<Command>,
    sim_contr_recv: Receiver<Command>,
    packet_recv: Receiver<Packet>,
    pdr: u8,
    packet_send: HashMap<NodeId, Sender<Packet>>,
}

impl Drone for MyDrone {
    fn new(
        id: NodeId,
        sim_contr_send: Sender<Command>,
        sim_contr_recv: Receiver<Command>,
        packet_recv: Receiver<Packet>,
        pdr: f32,
    ) -> Self {
        Self {
            id,
            sim_contr_send,
            sim_contr_recv,
            packet_recv,
            pdr: (pdr * 100.0) as u8,
            packet_send: HashMap::new(),
        }
    }

    fn run(&mut self) {
        self.run_internal();
    }
}

impl MyDrone {
    fn run_internal(&mut self) {
        loop {
            select! {
                recv(self.get_packet_receiver()) -> packet_res => {
                    if let Ok(packet) = packet_res {
                    // each match branch may call a function to handle it to make it more readable
                        match packet.pack_type {
                            PacketType::Nack(nack) => todo!(),
                            PacketType::Ack(ack) => todo!(),
                            PacketType::MsgFragment(fragment) => todo!()
                        }
                    }
                },
                recv(self.get_sim_controller_receiver()) -> command_res => {
                    if let Ok(command) = command_res {
                        //handle the simulation controller's command
                    }
                }
            }
        }
    }

    fn add_channel(&mut self, id: NodeId, sender: Sender<Packet>) {
        self.packet_send.insert(id, sender);
    }

    // fn remove_channel(...) {...}
}
