/*pub enum NodeType{
    Client(u64), MediaServer(u64), TextServer(u64), Drone(u64)
}
 */
use crate::types::packet::{Fragment, Packet};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
//use crate::types::message::Message;
use crate::types::routing::Query;
type NodeId = u64;

pub struct Node {
    name: NodeId,
    node_type: NodeType,
    neighbors: HashMap<NodeId, NodeRef>, //node ids
}
pub enum NodeType {
    Client(NodeId),
    MediaServer(NodeId),
    TextServer(NodeId),
    Drone(NodeId),
}
type NodeRef = Rc<RefCell<Node>>;
//impl NetAble for NodeRef{...}
struct Route {
    //route is the actual vec of references, path is just a "treasure map" with ids
    path: Vec<NodeRef>,
}
type Routes = HashMap<NodeId, Route>;
//Routes are only from client to server and vice versa.
type Path = Vec<NodeId>; //collection of node ids

pub trait NetAble {
    fn send_packet(&self, path: &Path, hop: usize, packet: Packet) -> bool;
    // forwards a packet and sends back an acknowledgement to the sender.
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
