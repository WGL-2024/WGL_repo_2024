use crate::types::topology::routes::Path;

pub struct Packet {}
pub trait DroneAble {
    fn forward_packet(&self, path: &Path, hop: usize, packet: Packet) -> bool;
}
