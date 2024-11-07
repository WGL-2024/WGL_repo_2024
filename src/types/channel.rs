use crossbeam_channel::{Receiver, Sender};
use crate::types::packet::Packet;

pub struct Channel{
    sender: Sender<Packet>,
    receiver: Receiver<Packet>,
}
trait ChannelTrait{
    fn new(sender: Sender<Packet>, receiver: Receiver<Packet>) -> Self;
}