//THIS IS JUST AN EXAMPLE OF IMPLEMENTATION

use crate::api::drone::{Drone, DroneImplement};
use crate::types::packet::{Packet, PacketType};
use crossbeam_channel::{select, Receiver, Sender};
use std::any::Any;
use std::collections::HashMap;
use std::thread;

fn main() {
    let handler = thread::spawn(move || {
        //Create some drone
        // let drone = Drone::new(...);

        //Then start the drone
        //(not forced to use '.run()', a loop is enough).
        // drone.run();
    });
}

impl DroneImplement for Drone {
    fn new(
        id: NodeId,
        scs: Sender<Command>,
        scr: Receiver<Command>,
        ps: HashMap<NodeId, Sender<Packet>>,
        pr: Receiver<Packet>,
        pdr: f32,
    ) -> Drone {
        Drone {
            drone_id: id,
            sim_contr_send: scs,
            sim_contr_recv: scr,
            packet_send: ps,
            packet_recv: pr,
            pdr: (pdr * 100.0) as u8,
        }
    }
}

impl Drone {
    fn run(&mut self) {
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
}
