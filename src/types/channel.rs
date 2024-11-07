use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crossbeam_channel::{Receiver, Sender};
use crate::types::packet::Packet;
use crate::types::srh::NodeId;

pub struct Channel{
    sender: Sender<Packet>,
    listener: Receiver<Packet>,
}
pub struct Channels{
    pub channels: Rc<RefCell<HashMap<NodeId, Channel>>>,
}

impl Channels{
    pub fn new() -> Self {
        Channels{
            channels: Rc::new(RefCell::new(HashMap::new())),
        }
    }
    pub fn add_channel(&mut self, drone_id: u8, new_channel: Channel) -> bool {
        unimplemented!()
    }
    pub fn remove_channel(&mut self, drone_id: u8) {
        unimplemented!()
    }
    pub fn get_map(&self) -> Rc<RefCell<HashMap<u8, Channel>>> {
        self.channels.clone()
    }
}