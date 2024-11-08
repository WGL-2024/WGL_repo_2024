use crate::types::channel::Channel;
use crate::types::NodeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

pub struct Drone {
    id: NodeId,
    thread: thread::JoinHandle<()>,
    channels: Arc<RefCell<HashMap<u8, Channel>>>,
}

trait DroneTrait {
    fn new(drone_id: u8) -> Self;
    // The thread would be created inside here,
    // giving him reference to the list of channels,
    // that would be expanded during the creation of the
    // later drones.
    fn add_channel(new_channel: Channel) -> bool;
    fn remove_channel(drone_id: u8) -> bool;
}
