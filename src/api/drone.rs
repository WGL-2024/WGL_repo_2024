use crate::types::topology::routes::Path;

pub struct Packet {}
pub trait DroneAble {
    fn forward_packet(&self, hop: usize, packet: Packet) -> bool;
}
