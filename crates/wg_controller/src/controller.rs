use wg_network::NodeId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType {
    Server,
    Client,
    Drone,
}

pub trait SimulationController {
    fn crash(&mut self, crashed: &str);
    fn spawn_node(&mut self, node_id: NodeId, node_type: NodeType /*metadata*/);
    fn message_sent(source: &str, target: &str /*metadata*/);
}
