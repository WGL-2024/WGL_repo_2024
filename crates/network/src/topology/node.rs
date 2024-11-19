use crate::routing::NodeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type NodeRef = Rc<RefCell<Node>>;

pub struct Topology {
    pub nodes: Vec<NodeRef>,
}

pub struct Node {
    pub name: NodeId,
    pub node_type: NodeType,
    pub neighbors: HashMap<NodeId, NodeRef>, //node ids
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
