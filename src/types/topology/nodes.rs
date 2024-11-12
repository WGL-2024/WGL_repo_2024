use crate::types::source_routing_header::NodeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type NodeRef = Rc<RefCell<Node>>;

pub struct Topology {
    nodes: Vec<NodeRef>,
}

pub struct Node {
    name: NodeId,
    node_type: NodeType,
    neighbors: HashMap<NodeId, NodeRef>, //node ids
}
pub enum NodeType {
    Client(NodeId),
    Server(ServerType, NodeId),
    Drone(NodeId),
}

#[derive(Debug)]
pub enum ServerType {
    Chat,
    Text,
    Media,
}
