use wg_network::topology::Node;

pub trait SimulationController {
    fn crash(&mut self, crashed: &str);
    fn spawn_node(&mut self, new_node: Node /*metadata*/);
    fn message_sent(source: &str, target: &str /*metadata*/);
}
