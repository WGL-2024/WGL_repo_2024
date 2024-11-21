#![allow(unused)]

use crossbeam_channel::{select, unbounded, Receiver, Sender};
use std::collections::HashMap;
use std::thread::{self, JoinHandle};
use wg_2024::config::Config;
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
            packet_send: options.packet_send,
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
                            PacketType::FloodRequest(_) => unimplemented!(),
                            PacketType::FloodResponse(_) => unimplemented!(),
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

    // act like the config is actually initialized
    let config: Config = todo!();

    // these hashmaps can then be stored in the simulation controller
    let mut packet_channels: HashMap<NodeId, (Sender<Packet>, Receiver<Packet>)> = HashMap::new();
    let mut command_channels: HashMap<NodeId, (Sender<Command>, Receiver<Command>)> =
        HashMap::new();

    let mut join_handles: Vec<JoinHandle<()>> = Vec::new();

    //
    // since the config doesn't use NodeId but u64 in this branch, you'll see conversions that won't be needed in the future
    //

    for drone in config.drone.iter() {
        //create unbounded channel for drones
        packet_channels.insert(drone.id as NodeId, unbounded::<Packet>());
        command_channels.insert(drone.id as NodeId, unbounded::<Command>());
    }

    for drone in config.drone.iter() {
        //clones all the sender channels for the connected drones
        let mut packet_send: HashMap<NodeId, Sender<Packet>> = HashMap::new();

        for connected_drone in drone.connected_drone_ids.iter() {
            packet_send.insert(
                *connected_drone as NodeId,
                packet_channels
                    .get(&(*connected_drone as NodeId))
                    .unwrap()
                    .0
                    .clone(),
            );
        }

        // clone the channels to give them to each thread
        let packet_recv = packet_channels.get(&(drone.id as u8)).unwrap().1.clone();
        let sim_contr_recv = command_channels.get(&(drone.id as u8)).unwrap().1.clone();
        let sim_contr_send = command_channels.get(&(drone.id as u8)).unwrap().0.clone();

        // since the thread::spawn function will take ownership of the values, we need to copy or clone the values from 'drone' since it's a borrow
        let id: NodeId = drone.id.try_into().unwrap();
        let pdr = drone.pdr as f32;

        join_handles.push(thread::spawn(move || {
            let mut drone = MyDrone::new(DroneOptions {
                id,
                sim_contr_recv,
                sim_contr_send,
                packet_recv,
                pdr,
                packet_send,
            });

            drone.run();
        }));
    }

    // here you'd create your simulation controller and also pass all the channels to it

    // joining behaviour needs to be refined
    join_handles[0].join().ok();
}
