use crate::types::topology::routes::Path;
use crate::types::topology::routes::{Route, Routes};

pub struct Query {}

pub trait ClientServerAble {
    fn find_routes(&self) -> Routes;
    //all routes from node where you start to other endpoints:
    //so from client to server and vice versa
    //calls look_around() in its implementation,
    //returning a route to each reachable endpoint
    fn look_around(&self, previous_paths: Vec<Path>) -> Vec<Path>;
    //finds all possible paths that a packet can take from a given node.
    //look_around discards routes that would cause cycles:
    //node ids are not repeated, to prevent paths that would go back to sender.
    fn discovery(&self, query: Query) -> Vec<Route>;
    //only the initiator node should call discovery.
    //for each node, finds all possible routes, based on the query
}
