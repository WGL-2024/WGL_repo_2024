use crate::routing::NodeId;
use crate::topology::NodeRef;
use std::collections::HashMap;

pub struct Route {
    /// Route is the actual Vec of references, path is just a "treasure map" with ids
    pub path: Vec<NodeRef>,
}

/// Routes are only from client to server and vice versa.
pub type Routes = HashMap<NodeId, Route>;

/// Path is a collection of NodeId
pub type Path = Vec<NodeId>;
