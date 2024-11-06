use std::thread;
use crate::types::channel::{Channel, Channels};
use crate::types::srh::NodeId;

#[allow(unused_variables)]
pub fn new_drone(drone_id: NodeId, channel_sc: Channel) -> thread::JoinHandle<()>{
    thread::spawn(move || {
        let id = drone_id;
        let mut channels = Channels::new();
        channels.add_channel(0, channel_sc);
        //ID=0 would be for the Simulation Controller

        loop{
            todo!();
        };
    })
}
