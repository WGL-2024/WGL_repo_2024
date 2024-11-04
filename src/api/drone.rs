use crate::types::topology::routes::Path;
use std::thread;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender; //still not sure about the types, didn't experiment much

pub fn

new_drone(drone_id: u8, _neighbors_id: Vec<u8>, chnl_recv: Vec<Receiver<Packet>>,
                 chnl_send: Vec<Sender<Packet>>)  {
    thread::spawn(move || {
        let _id = drone_id;
        let receivers = chnl_recv;
        let _senders = chnl_send;

        for i in &receivers {
            i.recv().unwrap();
        } //This would listen to all the channel for which this drone is a receiver
    });
}
pub struct Packet {}
pub trait DroneAble {
    fn forward_packet(&self, path: &Path, hop: usize, packet: Packet) -> bool;
}
