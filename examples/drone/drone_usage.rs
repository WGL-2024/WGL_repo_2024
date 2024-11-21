#![allow(unused)]

use crossbeam_channel::{select, Receiver, Sender};
use std::collections::HashMap;
use std::thread;
use wg_2024::controller::Command;
use wg_2024::drone::Drone;
use wg_2024::network::NodeId;
use wg_2024::packet::{Packet, PacketType};
use wg_internal::drone::DroneOptions;

/// Example of drone implementation
struct MyDrone {
    id: NodeId,
    sim_contr_send: Sender<Command>,
    sim_contr_recv: Receiver<Command>,
    packet_recv: Receiver<Packet>,
    pdr: u8,
    packet_send: HashMap<NodeId, Sender<Packet>>,
}

impl Drone for MyDrone {
    fn new(options: DroneOptions) -> Self {
        Self {
            id: options.id,
            sim_contr_send: options.sim_contr_send,
            sim_contr_recv: options.sim_contr_recv,
            packet_recv: options.packet_recv,
            pdr: (options.pdr * 100.0) as u8,
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
                recv(self.packet_recv) -> packet_res => {
                    if let Ok(packet) = packet_res {
                    // each match branch may call a function to handle it to make it more readable
            match packet.pack_type {
                            PacketType::Nack(_nack) => unimplemented!(),
                            PacketType::Ack(_ack) => unimplemented!(),
                            PacketType::MsgFragment(_fragment) => unimplemented!(),
                            PacketType::Flood(_) => unimplemented!(),
                            PacketType::FloodResult(_) => unimplemented!(),
                        }
                    }
                },
                recv(self.sim_contr_recv) -> command_res => {
                    if let Ok(_command) = command_res {
                        // handle the simulation controller's command
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

fn main() {
    // Something like this will be done
    // by the initialization controller
    let handler = thread::spawn(move || {
        let id = 1;
        let (sim_contr_send, sim_contr_recv) = crossbeam_channel::unbounded();
        let (_packet_send, packet_recv) = crossbeam_channel::unbounded();
        let mut drone = MyDrone::new(DroneOptions {
            id,
            sim_contr_recv,
            sim_contr_send,
            packet_recv,
            pdr: 0.1,
        });

        drone.run();
    });
    handler.join().ok();
}
