use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crossbeam_channel::{Receiver, Sender};
use crate::types::packet::Packet;
use crate::types::sourceRoutingHeader::NodeId;

pub struct Channel{
    sender: Sender<Packet>,
    receiver: Receiver<Packet>,
}
trait ChannelTrait{
    fn new(sender: Sender<Packet>, receiver: Receiver<Packet>) -> Self;
}