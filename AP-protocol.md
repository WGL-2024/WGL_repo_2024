# [Faulty] The communication protocol specifications

This document provides the specifications of the communication protocol used by the drones, the client and the servers of the network. In the following document, drones, clients and servers are collectively referred to as **nodes**. The specifications are often broken or incomplete, you must improve over them.

This document also establishes some technical requirements of the project.

# Types used in this document
Can be useful for understanding and for not having to change the underlining type everywhere.

```rust
type NodeId = u64;
```

# Network Initializer

The **Network Initializer**:
1. reads a local **Network Initialization File** that encodes the network topology and the drone parameters
2. checks that the initialization file adheres to the formatting and restrictions defined in the section below
3. checks that the initialization file represents a bidirectional graph
4. according to the network topology, defined in the initialization file, performs the following actions(in no particular order):
   - initializes the drones, distributing the implementations bought from the other groups(`impl`) as evenly as possible, having at most a difference of 1 between the group with the most drones running and the one with the least:
		- for 10 drones and 10 `impl`, 1 distinct `impl` for each drone
		- for 15 drones and 10 `impl`, each `impl` should be used at least once
		- for 5 drones and 10 `impl`, only some of the `impl` will be used
		- for 10 drones and 1 `impl`, all drones will have that `impl` 
   - sets up the Rust channels for communicating between nodes that are connected in the topology
   - sets up the Rust channels for communication between nodes and the simulation controller
   - spawns the node threads
   - spawns the simulation controller thread


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

### Additional requirements
- note that the **Network Initialization File** should never contain two **nodes** with the same `id` value
- Note that the **Network Initialization File** does not define if a drone should use a particular implementation, every group is expected to import the drones they bought at the fair in the Network Initializer, and distribute them as explained in the previous section 

# Drone parameters: Packet Drop Rate

A drone is characterized by a parameter that regulates what to do when a packet is received, that thus influences the simulation. This parameter is provided in the Network Initialization File.

Packet Drop Rate: The drone drops the received packet with probability equal to the Packet Drop Rate.

The PDR can be up to 100%, and the routing algorithm of every group should find a way to eventually work around this.

# Messages and fragments

Recall that there are: Content servers (that is, Text and Media servers) and Communication servers. These servers are used by clients to implement applications.

These servers exchange, respectively, Text server messages, Media server messages and Communication server messages. These are high-level messages. Recall that you must standardize and regulate their low-level counterparts (that is, fragments).

# Source Routing Protocol

The fragments that circulate in the network are **source-routed** (except for the commands sent from and the events received by the Simulation Controller).

Source routing refers to a technique where the sender of a data packet specifies the route the packet takes through the network. This is in contrast with conventional routing, where routers in the network determine the path incrementally based on the packet's destination.

The consequence is that drones do not need to maintain routing tables.

### How Source Routing Works

When a client or server wants to send a message to another node, it performs the following steps:

- **Route Computation**: The sender calculates the entire path to the destination node. This path includes the sender itself and all intermediate nodes leading to the destination.

- **Creation of the Source Routing Header**: The sender constructs a header that contains:
	- **`hops`**: A list of node IDs representing the route from the sender to the destination.
	- **`hop_index`**: An index indicating the current position in the `hops` list. It starts at **1** because the first hop (`hops[0]`) is the sender itself.

- **Packet Sending**: The sender attaches the source routing header to the packet and sends it to the first node in the route (the node at `hops[1]`).

Note: it is not mandatory to follow this precise order.

### Step-by-Step Example

Consider the following simplified network topology:

![constellation](assets/costellation.png)

Suppose that client A wants to send a message to server D.

**Client A**:

- Computes the route: **A → B → E → F → D**.
- Creates a source routing header:
	- **`hops`**: `[A, B, E, F, D]`.
	- **`hop_index`**: `1`.
- Sends the packet to **B**, the first node after itself.

**At Each Node**:

1. **Node B**:
	- Receives the packet.
	- Determines that the next hop is **E**.
	- Sends the packet to **E**.

2. **Node E**:
	- Receives the packet.
	- Determines that the next hop is **F**.
	- Sends the packet to **F**.

3. **Node F**:
	- Receives the packet.
	- Determines that the next hop is **D**.
	- Sends the packet to **D**.

4. **Node D**:
	- Receives the packet.
	- Sees that there are no more hops in the route.
	- Processes the packet as the **final destination**.

For detailed steps on how each drone processes packets, including verification, error handling, and forwarding, please refer to the [Drone Protocol](#drone-protocol) section.


```rust
struct SourceRoutingHeader {
	// must be set to 1 initially by the sender
	hop_index: usize,
	// Vector of nodes with initiator and nodes to which the packet will be forwarded to.
	hops: Vec<NodeId>
}
```

## Network **Discovery Protocol**

When the network is first initialized, nodes only know who their own neighbors are.

Clients and servers need to obtain an understanding of the network topology ("what nodes are there in the network and what are their types?") so that they can compute a route that packets take through the network (refer to the Source routing section for details).

To do so, they must use the **Network Discovery Protocol**. The Network Discovery Protocol is initiated by clients and servers and works through query flooding.

### **Flooding Initialization**

The client or server that wants to learn the topology, called the **initiator**, starts by flooding a query to all its immediate neighbors:

```rust
enum NodeType {Client, Drone, Server}

struct FloodRequest {
	/// Unique identifier of the flood, to prevent loops.
	flood_id: u64,
	/// ID of client or server
	initiator_id: NodeId,
	/// Records the nodes that have been traversed (to track the connections).
	path_trace: Vec<(NodeId, NodeType)>
}
```

### **Neighbor Response**

When a neighbor node receives the flood request, it processes it based on the following rules:
- If the flood ID has already been received:
	- The drone adds itself to the `path_trace`.
	- The drone creates a `FloodResponse` and sends it back.

- If the flood ID has not yet been received:
	- The drone adds itself to the `path_trace`.
	- **If it has neighbors** (excluding the one from which it received the `FloodRequest`):
		- The drone forwards the packet to its neighbors (except the one from which it received the `FloodRequest`).
	- **If it has no neighbors**, then:
		- The drone creates a `FloodResponse` and sends it to the node from which it received the `FloodRequest`.

```rust
struct FloodResponse {
	flood_id: u64,
	path_trace: Vec<(NodeId, NodeType)>
}
```

#### Notes:
- For the discovery protocol, `Packet`s of type `FloodRequest` and `FloodResponse` will be sent.
- The `routing_header` of `Packet`s of type `FloodRequest` will be ignored (as the Packet is sent to all neighbors except the one from which it was received).
- The `routing_header` of `Packet`s of type `FloodResponse`, on the other hand, determines the packet's path.

### **Recording Topology Information**

For every flood response the initiator receives, it updates its understanding of the graph:

- If the node receives a flood response with a **path trace**, it records the paths between nodes. The initiator learns not only the immediate neighbors but also the connections between nodes further out.
- Over time, as the query continues to flood, the initiator accumulates more information and can eventually reconstruct the entire graph's topology.

### **Termination Condition**

The flood can terminate when:
- A node receives a `FloodRequest` with a flood_id that has already been received.
- A node receives a `FloodRequest` but has no neighbors to forward the request to.


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
pub struct Ack {
	fragment_index: u64,
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

When a drone receives a packet, it **must** perform the following steps:

1. **Step 1**: Check if `hops[hop_index]` matches the drone's own `NodeId`.
	- **If yes**, proceed to Step 2.
	- **If no**, send a Nack with `UnexpectedRecipient` (including the drone's own `NodeId`) and terminate processing.

2. **Step 2**: Increment `hop_index` by **1**.

3. **Step 3**: Determine if the drone is the final destination:
	- **If `hop_index` equals the length of `hops`**, the drone is the final destination send a Nack with `DestinationIsDrone` and terminate processing.
	- **If not**, proceed to Step 4.

4. **Step 4**: Identify the next hop using `hops[hop_index]`, let's call it `next_hop`.
	- **If `next_hop` is not a neighbor** of the drone, send a Nack with `ErrorInRouting` (including the problematic `NodeId` of `next_hop`) and terminate processing.
	- **If `next_hop` is a neighbor**, proceed to Step 5.

5. **Step 5**: Proceed based on the packet type:

   - **Flood Messages**: If the packet is flood-related, follow the rules specified in the **Network Discovery Protocol** section.

   - **`MsgFragment`**:

      a. **Check for Packet Drop**:
      - Determine whether to drop the packet based on the drone's **Packet Drop Rate (PDR)**.

      b. **If the packet is to be dropped**:
      - Send back a Nack with type `Dropped`. The Nack should have a Source Routing Header containing the reversed path from the current drone back to the sender.
      - Terminate processing.

      c. **If the packet is not to be dropped**:
      - Send the packet to `next_hop` using the appropriate channel.


### Step-by-Step Example

Consider the following simplified network topology:

![constellation](assets/costellation.png)

Suppose that client A wants to send a message to server D.

**Client A**:

- Computes the route: **A → B → E → F → D**.
- Creates a source routing header:
	- **`hops`**: `[A, B, E, F, D]`.
	- **`hop_index`**: `1`.
- Sends the packet to **B**, the first node after itself.

**Detailed Steps**:

1. **Node B**:
	- Receives the packet with `hop_index = 1`.
	- Checks: `hops[1] = B` matches its own ID.
	- Increments `hop_index` to `2`.
	- Next hop is `hops[2] = E`.
	- Sends the packet to **E**.

2. **Node E**:
	- Receives the packet with `hop_index = 2`.
	- Checks: `hops[2] = E` matches its own ID.
	- Increments `hop_index` to `3`.
	- Next hop is `hops[3] = F`.
	- Sends the packet to **F**.

3. **Node F**:
	- Receives the packet with `hop_index = 3`.
	- Checks: `hops[3] = F` matches its own ID.
	- Increments `hop_index` to `4`.
	- Next hop is `hops[4] = D`.
	- Sends the packet to **D**.

4. **Node D**:
	- Receives the packet with `hop_index = 4`.
	- Checks: `hops[4] = D` matches its own ID.
	- Increments `hop_index` to `5`.
	- Since `hop_index` equals the length of `hops`, there are no more hops.
	- Concludes it is the **final destination** and processes the packet.


## Simulation
TODO


# Simulation Controller

Like nodes, the **Simulation Controller** (SC) runs on a thread. It must retain a means of communication with all nodes of the network, even when drones go down. 
The Simulation controller can send and receive different commands to/from the nodes (drones, clients and servers) through reserved channels. The list of available **commands** is as follows:

```rust
/// From controller to drone
#[derive(Debug, Clone)]
pub enum DroneCommand {
    AddSender(NodeId, Sender<Packet>),
    SetPacketDropRate(f32),
    Crash,
}

/// From drone to controller
#[derive(Debug, Clone)]
pub enum NodeEvent {
    PacketSent(Packet),
    PacketDropped(Packet),
}
```

The Simulation Controller can execute the following tasks:

`Spawn`: This command adds a new drone to the network.

### Simulation commands

The Simulation Controller can send the following commands to drones:

`Crash`: This command makes a drone crash. Upon receiving this command, the drone’s thread should return as soon as possible.

`AddSender(dst_id, crossbeam::Sender)`: This command adds `dst_id` to the drone neighbors, with `dst_id` crossbeam::Sender.

`SetPacketDropRate(pdr)`: This command alters the pdr of a drone.

### Simulation events

The Simulation Controller can receive the following events from nodes:

`PacketSent(packet)`: This event indicates that node has sent a packet. All the informations about the `src_id`, `dst_id` and `path` are stored in the packet routing header.

`PacketDropped(packet)`: This event indicates that node has dropped a packet. All the informations about the `src_id`, `dst_id` and `path` are stored in the packet routing header.

## Note on commands and events

Due to the importance of these messages, drones MUST prioritize handling commands from the simulation controller over messages and fragments.

This can be done by using [the select_biased! macro](https://shadow.github.io/docs/rust/crossbeam/channel/macro.select_biased.html) and putting the simulation controller channel first, as seen in the example.



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
