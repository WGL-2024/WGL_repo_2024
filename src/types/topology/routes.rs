use crate::types::topology::{NodeId, NodeRef};
use std::collections::HashMap;

pub struct Route {
    //route is the actual vec of references, path is just a "treasure map" with ids
    path: Vec<NodeRef>,
}
pub type Routes = HashMap<NodeId, Route>;
//Routes are only from client to server and vice versa.
pub type Path = Vec<NodeId>; //collection of node ids
