use crate::types::node::Node;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Topology {
    nodes: Vec<NodeRef>,
    //node already contains a hashmap with its neighbors and its own node type
}
type NodeRef = Rc<RefCell<Node>>;
impl Topology {
    pub fn new(nodes: Vec<NodeRef>) -> Self {
        unimplemented!()
    }
    pub fn crash(&mut self, crashed: &str) {
        unimplemented!()
    }
    pub fn spawn_node(&mut self, new_node: Node /*metadata*/) {
        unimplemented!()
    }
    pub fn message_sent<'a>(source: &'a str, target: &'a str /*metadata*/) {
        unimplemented!()
    }
}
