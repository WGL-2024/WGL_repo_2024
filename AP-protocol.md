# [Faulty] The communication protocol specifications

This document provides the specifications of the communication protocol used by the drones, the client and the servers of the network. In the following document, drones, clients and servers are collectively referred to as **nodes**. The specifications are often broken or incomplete, you must improve over them.

This document also establishes some technical requirements of the project.

# Types used in this document
Can be useful for understanding and for not having to change the underlining type everywhere.

```rust
type NodeId = u8;
```

# Network Initializer

The **Network Initializer** reads a local **Network Initialization File** that encodes the network topology and the drone parameters and, accordingly, spawns the node threads and sets up the Rust channels for communicating between nodes.

> Importantly, the Network Initializer should also set up the Rust channels between the nodes and the Simulation Controller (see the Simulation Controller section).

## Network Initialization File
The **Network Initialization File** is in the `.toml` format, and structured as explained below:

### Drones
Any number of drones, each formatted as:
```TOML
[[drone]]
id = "drone_id"
connected_node_ids = ["connected_id1", "connected_id2", "connected_id3", "..."]
pdr = "pdr"
```
- note that the `pdr` is defined between 0 and 1 (0.05 = 5%).
- note that `connected_node_ids` cannot contain `drone_id` nor repetitions

### Clients
Any number of clients, each formatted as:
```TOML
[[client]]
id = "client_id"
connected_drone_ids = ["connected_id1", "..."] # max 2 entries
```
- note that `connected_drone_ids` cannot contain `client_id` nor repetitions
- note that a client cannot connect to other clients or servers
- note that a client can be connected to at least one and at most two drones

### Servers
Any number of servers, each formatted as:
```TOML
[[server]]
id = "server_id"
connected_drone_ids = ["connected_id1", "connected_id2", "connected_id3", "..."] # at least 2 entries
```
- note that `connected_drone_ids` cannot contain `server_id` nor repetitions
- note that a server cannot connect to other clients or servers
- note that a server should be connected to at least two drones

# Drone parameters: Packet Drop Rate

A drone is characterized by a parameter that regulates what to do when a packet is received, that thus influences the simulation. This parameter is provided in the Network Initialization File.

Packet Drop Rate: The drone drops the received packet with probability equal to the Packet Drop Rate.

The PDR can be up to 100%, and the routing algorithm of every group should find a way to eventually work around this.

# Messages and fragments

Recall that there are: Content servers (that is, Text and Media servers) and Communication servers. These servers are used by clients to implement applications.

These servers exchange, respectively, Text server messages, Media server messages and Communication server messages. These are high-level messages. Recall that you must standardize and regulate their low-level counterparts (that is, fragments).

# Source routing

The fragments that circulate in the network are **source-routed** (except for the commands sent from and the events received by the Simulation Controller).

Source routing refers to a technique where the sender of a data packet specifies the route the packet takes through the network. This is in contrast with conventional routing, where routers in the network determine the path incrementally based on the packet's destination.

The consequence is that drones do not need to maintain routing tables.

As an example, consider the following simplified network:

![constellation](assets/costellation.png)

Suppose that the client A wants to send a message to the server D.

It computes the route B→E→F→D, creates a **Source Routing Header** specifying route A→B→E→F→D, adds it to the packet and sends it to B.

When B receives the packet, it sees that the next hop is E and sends the packet to it.

When E receives the packet, it sees that the next hop is F and sends the packet to it.

When F receives the packet, it sees that the next hop is D and sends the packet to it.

When D receives the packet, it sees there are no more hops so it must be the final destination: it can thus process the packet.

```rust
struct SourceRoutingHeader {
	// must be set to 0 initially by the sender
	hop_index: usize,
	// Vector of nodes with initiator and nodes to which the packet will be forwarded to.
	hops: Vec<NodeId>
}
```

## Network **Discovery Protocol**

When the network is first initialized, nodes only know who their own neighbors are.

Clients and servers need to obtain an understanding of the network topology ("what nodes are there in the network and what are their types?") so that they can compute a route that packets take through the network (refer to the Source routing section for details).

To do so, they must use the **Network Discovery Protocol**. The Network Discovery Protocol is initiated by clients and servers and works through query flooding, or by newly connected drones.

### **Flooding Initialization**

The client or server that wants to learn the topology (or a path to a certain node), called the **initiator**, starts by flooding a query to all its immediate neighbors:

```rust
pub struct FloodRequest {
	// The flood ID, randomly generated to prevent clashes between floods.
    pub flood_id: u64,
	// The initiator's ID
    pub prev_hop: NodeId,
	// If for some reason the initiator wants to ONLY find a path to a specific NodeId, it can set this field
    pub to: Option<NodeId>,
	// MUST be set to 'standard' unless this flood is triggered due to a crash, in which case it SHOULD be set to 'PreviousFloodFailed'
    pub reason: FloodReason

}
```


### **Neighbor response and forwarding**

When a neighbor receives a flood request with a certain ID for the first time, it saves the flood ID and previous hop internally, amd it MUST return a `FloodProcessing` response with its own node ID as the `processing_node_id` and the current flood ID.

The node then swaps `prev_hop` in the received `FloodRequest` with itself and proceeds to send it to all of its neighbors except the one it just received it from.

The node then waits for replies, and notes down every node from which it receives a `FloodProcessing` response as **pending** in the current flood request.

In case a neighbor crashes (AKA a notification from the controller to drop that channel is received), that node is marked as **failed**

If the node is seeing a `FloodRequest` with the same flood ID it has already seen, it MUST ignore the request.

### **Replies, merging and termination condition**

When a **pending** neighbor returns a `FloodData` message, the message is temporarily saved and the node is marked as **done**. When all neighbors are **done** or **failed**, or if there are none (in the case of a node connected to only one neighbor, from which it has received the request) the Node creates its own `FloodData`:
```rust
pub struct FloodData {
	// The original flood ID
    pub flood_id: u64,
	// Its own node ID
	pub from:  NodeId
	// Its own node ID and type
    pub node_list: HashSet<(NodeId, NodeType)>,
	// See below for explaination
    pub connections: HashSet<(NodeId, NodeId)>,
	// The Node ID of one of the nodes marked as "failed", otherwise None
    pub error_at: Option<NodeId>,
	// Kept at "false" 
    pub is_broadcast: bool 
}
```
The `connections` field gets populated with the neighbors, creating a tuple of two node IDs, one being the current and one being the neighbor, which MUST be in ascending order: (n1,n2) where n1 < n2 always.
The node then proceeds to merge all obtained responses, and MUST send the final response to the previous hop it had stored, changing the `from` field to its own ID.

### **Special considerations for Requests containing a "to" field**
In this case, when returning a `FloodData` packet, the fields `connections` and `node_list` get set to empty, unless:
	- The node is the target node, in which case it MUST populate `node_list` with itself, and MUST add the connection to the previous hop on the request to `connections`, or
	- The node is not the target node but has received a non-empty `FloodData`, in which case it MUST add the connection to the previous hop as described above, and CAN add itself to the `node_list`.
Nodes SHOULD avoid waiting for all the **pending** nodes, immediately sending a `FloodData` message (since other replies would be either empty or represent longer paths).

### **Broadcast messages**

When a drone first connects to the network, it creates a `FloodData` message, containing a random `flood_id`, itself as the `node_list` and its neighbors as `connections`, with `is_broadcast` set to true. This message SHOULD get forwarded to neighbouring nodes. When receiving such a message a node MUST forward it to every other neighbor except the one it received it from. If received a second time the message MUST get ignored.

### **Optional considerations**
Nodes can speed up the processing of packets containing the "to" field if they're connected to the target node by forwarding the request only to that node.
If a server or client receives a `FloodRequest` with `FloodReason` set to `PreviousFloodFailed`, it can use it as a heuristic to indicate that it must relearn the topology.

# **Client-Server Protocol: Fragments**

Clients and servers operate with high level `Message`s which are disassembled into atomically sized packets that are routed through the drone network. The Client-Server Protocol standardizes and regulates the format of these messages and their exchange.

The previously mentioned packets can be: Fragment, Ack, Nack, FloodRequest, FloodResponse.

As described in the main document, `Message`s must be serialized and can be possibly fragmented, and the `Fragment`s can be possibly dropped by drones.

### Message

`Message` is subject to fragmentation: see the dedicated section.

`Fragment` (and `Fragment` only) can be dropped by drones.

```rust
#[derive(Debug)]
pub enum ServerType {
	ChatServer,
	TextServer,
	MediaServer,
}

#[derive(Debug)]
pub struct Message {
	message_data: MessageData,
	routing_header: SourceRoutingHeader
}

#[derive(Debug)]
// Part to be fragmented
pub struct MessageData {
	session_id: u64,
	content: MessageContent
}
```

### Ack

If a drone receives a Message and can forward it to the next hop, it also sends an Ack to the client.

```rust
pub struct Ack{
	fragment_index: u64,
	time_received: std::time::Instant
}
```

### Nack
If an error occurs, then a Nack is sent. A Nack can be of type:
1. **ErrorInRouting**: If a drone receives a Message and the next hop specified in the Source Routing Header is not a neighbor of the drone, then it sends Error to the client.
2. **Dropped**: If a drone receives a Message that must be dropped due to the Packet Drop Rate, then it sends Dropped to the client.

Source Routing Header contains the path to the client, which can be obtained by reversing the list of hops contained in the Source Routing Header of the problematic Message.

This message cannot be dropped by drones due to Packet Drop Rate.

```rust
pub struct Nack {
	fragment_index: u64,
	time_of_fail: std::time::Instant,
	nack_type: NackType
}

pub enum NackType {
	ErrorInRouting(NodeId), // contains id of not neighbor
	DestinationIsDrone,
	Dropped
}
```

Source Routing Header contains the path to the client, which can be obtained by reversing the list of hops contained in the Source Routing Header of the problematic Message.

### Serialization

As described in the main document, Message fragment cannot contain dynamically-sized data structures (that is, **no** `Vec`, **no** `String`, etc.). Therefore, packets will contain large, fixed-size arrays instead.

### Fragment reassembly

```rust
// defined as atomic message exchanged by the drones.
pub struct Packet {
	pack_type: PacketType,
	routing_header: SourceRoutingHeader,
	session_id: u64,
}

pub enum PacketType {
	MsgFragment(Fragment),
	Ack(Ack),
	Nack(Nack),
	FloodRequest(FloodRequest),
	FloodResponse(FloodResponse),
}

// fragment defined as part of a message.
pub struct Fragment {
	fragment_index: u64,
	total_n_fragments: u64,
	length: u8,
	// assembler will fragment/de-fragment data into bytes.
	data: [u8; 80] // usable for image with .into_bytes()
}
```

To reassemble fragments into a single packet, a client or server uses the fragment header as follows:

1. The client or server receives a fragment.

2. It first checks the `session_id` in the header.

3. If it has not received a fragment with the same `session_id`, then it creates a vector (`Vec<u8>` with capacity of
   `total_n_fragments` * 80) where to copy the
   data of the fragments;

4. It would then copy `length` elements of the `data` array at the correct offset in the vector.

> Note: if there are more than one fragment, `length` must be 80 for all fragments except for the last.

If the client or server has already received a fragment with the same `session_id`, then it just needs to copy the data of the fragment in the vector.

Once that the client or server has received all fragments (that is, `fragment_index` 0 to `total_n_fragments` - 1), then it has reassembled the whole fragment.

Therefore, the packet is now a message that can be delivered.

# Drone Protocol
When a drone receives a packet, it **must** do the following:

1. increase `hop_index` by 1
2. obtain the (new `hop_index`) + 1 element of the `SourceRoutingHeader` vector `hops`, let's call it `next_hop`
	* It **must ignore** intentionally to check `hop_index`.

3. if `next_hop`
	* doesn't exist create a new packet of type Nack, precisely of type `DestinationIsDrone`. The packet must have the routing made of a vector but inverted and only contains the nodes from this drone to the sender. Send this packet as a normal packet. End here.
	* if the `NodeId` is not a neighbor, then creates a new packet of type Nack, precisely of type `ErrorInRouting` with field the value of `NodeId` of next hop. Continue as other error.

4. Proceed as follows based on packet type:

### Flood Messages
Refer to the Network Initialization section of the protocol.

### Normal Messages
1. check whether to drop or not the package based on the PDR,

2. based on if the packets need to be dropped or not do:

	* If is dropped, send back a Nack Packet with type `Dropped`. Follow the rules for sending errors as before.

	* If it is not dropped, send the packets using the channel relative to the next hops in `SourceRoutingHeader`.

## Simulation
TODO


# Simulation Controller

Like nodes, the **Simulation Controller** runs on a thread. It must retain a means of communication with all nodes of the network, even when drones go down.

### Simulation commands

The Simulation Controller can send the following commands to drones:

`Crash`: This commands makes a drone crash. Upon receiving this command, the drone’s thread should return as soon as possible.

`AddSender(crossbeam::Sender, dst_id)`: This command provides a node with a new crossbeam Sender to send messages to node `dst_id`.

`AddReceiver(mpsc::Receiver, src_id)`: This command provides a node with a new crossbeam Receiver to receive messages from node `src_id`.

`Spawn(id, code)`: This command adds a new drone to the network.

`SetPacketDropRate(id, new_pdr)`:

### Simulation events

The Simulation Controller can receive the following events from nodes:

`Topology(node_id, list_of_connected_ids, metadata)`: This event indicates that node `node_id` has been added to the network and its current neighbors are `list_of_connected_ids`. It can carry metadata that could be useful to display, such as the PDR and DR of Drones.

`MessageSent(node_src, node_trg, metadata)`: This event indicates that node `node_src` has sent a message to `node_trg`. It can carry useful metadata that could be useful display, such as the kind of message, that would allow debugging what is going on in the network.


# **Client-Server Protocol: High-level Messages**

These are the kinds of high-level messages that we expect can be exchanged between clients and servers.

Notice that these messages are not subject to the rules of fragmentation, in fact, they can exchange Strings, `Vecs` and other dynamically-sized types

#### Message Types
```rust
#[derive(Debug)]
pub enum MessageContent {
	// Client -> Server
	ReqServerType,
	ReqFilesList,
	ReqFile(u64),
	ReqMedia(u64),

	ReqClientList,
	ReqRegistrationToChat,
	ReqMessageSend { to: NodeId, message: Vec<u8> },

	// Server -> Client
	RespServerType(ServerType),
	RespFilesList(Vec<u64>),
	RespFile(Vec<u8>),
	RespMedia(Vec<u8>),
	ErrUnsupportedRequestType,
	ErrRequestedNotFound,

	RespClientList(Vec<NodeId>),
	RespMessageFrom { from: NodeId, message: Vec<u8> },
	ErrWrongClientId,
}
```

Example of new file request, with id = 8:
```rust
fn new_file_request(source_id: NodeId, session_id: u64, routing: SourceRoutingHeader) -> Message {
	let content = MessageType::ReqFile(8);
	Message::new(routing, source_id, session_id, content)
}
```
