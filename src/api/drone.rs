use crate::types::packet::Packet;

pub trait DroneAble {
    fn forward_packet(&self, packet: Packet) -> bool;
}
