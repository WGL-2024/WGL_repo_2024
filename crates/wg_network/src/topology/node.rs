use crate::routing::NodeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type NodeRef = Rc<RefCell<Node>>;

#[derive(Debug, Clone)]
pub struct Topology {
    pub nodes: Vec<NodeRef>,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: NodeId,
    pub node_type: NodeType,
    pub neighbors: HashMap<NodeId, NodeRef>, //node ids
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType {
    Client(NodeId),
    Server(ServerType, NodeId),
    Drone(NodeId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerType {
    Chat,
    Text,
    Media,
}
