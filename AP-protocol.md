# The communication protocol specifications

This document provides the specifications of the communication protocol used by the drones, the client and the servers of the network. In the following document, drones, clients and servers are collectively referred to as **nodes**.

This document also establishes some technical requirements of the project.

# Types used in this document
Can be useful for understanding and for not having to change the underlining type everywhere.

```rust
type NodeId = u8;
```

# Network Initializer

The **Network Initializer**:
1. reads a local **Network Initialization File** that encodes the network topology and the drone parameters
2. checks that the initialization file adheres to the formatting and restrictions defined in the section below

3. according to the network topology, defined in the initialization file, performs the following actions(in no particular order):
  - initializes the drones, distributing the implementations bought from the other groups(`impl`) as evenly as possible, having at most a difference of 1 between the group with the most drones running and the one with the least:
    - for 10 drones and 10 `impl`, 1 distinct `impl` for each drone
    - for 15 drones and 10 `impl`, each `impl` should be used at least once
    - for 5 drones and 10 `impl`, only some of the `impl` will be used
    - for 10 drones and 1 `impl`, all drones will have that `impl` 
  - sets up the Rust channels for communicating between nodes that are connected in the topology
  - sets up the Rust channels for communication between nodes and the simulation controller
  - spawns the node threads
  - spawns the simulation controller thread

  > Note that all the channels created by the Network Initializer, and by extension also those created by the Simulation Controller during the simulation, must be `unbounded` channels

## Network Initialization File
The **Network Initialization File** is in the `.toml` format, and structured as explained below:

### Drones
Any number of drones, each formatted as:
```toml
[[drone]]
id = "drone_id"
connected_node_ids = ["connected_id1", "connected_id2", "connected_id3", "..."]
pdr = "pdr"
```
- note that the `pdr` is defined between 0 and 1 (0.05 = 5%).
- note that `connected_node_ids` cannot contain `drone_id` nor repetitions

### Clients
Any number of clients, each formatted as:
```toml
[[client]]
id = "client_id"
connected_drone_ids = ["connected_id1", "..."] # max 2 entries
```
- note that `connected_drone_ids` cannot contain `client_id` nor repetitions
- note that a client cannot connect to other clients or servers
- note that a client can be connected to at least one and at most two drones

### Servers
Any number of servers, each formatted as:
```toml
[[server]]
id = "server_id"
connected_drone_ids = ["connected_id1", "connected_id2", "connected_id3", "..."] # at least 2 entries
```
- note that `connected_drone_ids` cannot contain `server_id` nor repetitions
- note that a server cannot connect to other clients or servers
- note that a server should be connected to at least two drones

### Additional requirements
- note that the **Network Initialization File** should never contain two **nodes** with the same `node_id` value
- note that the **Network Initialization File** should represent a **connected** and **bidirectional** graph
- note that the **Network Initialization File** should represent a network where clients and servers are at the edges of the network, which means that the graph obtained by removing clients and servers is still a **connected** graph
- note that the **Network Initialization File** does not define if a drone should use a particular implementation, every group is expected to import the drones they bought at the fair in the Network Initializer, and distribute them as explained in the previous section

# Drone parameters: Packet Drop Rate

A drone is characterized by a parameter that regulates what to do when a packet is received, that thus influences the simulation. This parameter is provided in the Network Initialization File.

Packet Drop Rate: The drone drops the received packet with probability equal to the Packet Drop Rate.

The PDR can be up to 100%, and the routing algorithm of every group should find a way to eventually work around this.

The only packet that can be dropped is the msg_fragment

# Messages and fragments

Recall that there are: Content servers (that is, Text and Media servers) and Communication servers. These servers are used by clients to implement applications.

These servers exchange, respectively, Text server messages, Media server messages and Communication server messages. These are high-level messages. Their low-level counterparts, that is fragment, are standardized inside the protocol.

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
 	- Increments the **`hop_index`** by 1.
	- Sends the packet to **E**.

2. **Node E**:
	- Receives the packet.
	- Determines that the next hop is **F**.
 	- Increments the **`hop_index`** by 1.
	- Sends the packet to **F**.

3. **Node F**:
	- Receives the packet.
	- Determines that the next hop is **D**.
 	- Increments the **`hop_index`** by 1.
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

Clients and servers need to obtain an understanding of the network topology ("what nodes are there in the network and what are their types") so that they can compute a route that packets take through the network (refer to the Source routing section for details).

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
	- **If it has no neighbors** (excluding the one from which it received the `FloodRequest`), then:
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
- The `routing_header` of `Packet`s of type `FloodResponse`, on the other hand, determines the packet's path, and is creared from the `path_trace` of the `FloodRequest` from which is generated.

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

The previously mentioned packets can be: `Fragment`, `Ack`, `Nack`, `FloodRequest`, `FloodResponse`.

As described in the main document, `Message`s must be serialized and can be possibly fragmented, and the `Fragment`s can be possibly dropped by drones.

### Ack

This Ack is sent by a **client/server** after receiving a Packet.

```rust
pub struct Ack {
	fragment_index: u64,
}
```

### Nack
If an error occurs, then a `Nack` is sent.
The NackTypes are described in the Drone Protocol section.

When a `Nack` is sent back the Source Routing Header contains the path to the src_id, which can be obtained by reversing the list of hops contained in the Source Routing Header of the problematic Packet.

This Packet cannot be dropped by drones due to Packet Drop Rate.

```rust
pub struct Nack {
	fragment_index: u64,
	nack_type: NackType
}

pub enum NackType {
    ErrorInRouting(NodeId), // contains id of not neighbor
    DestinationIsDrone,
    Dropped,
    UnexpectedRecipient(NodeId),
}
```

Source Routing Header contains the path to the client, which can be obtained by reversing the list of hops contained in the Source Routing Header of the problematic Message.

### Limit Case
If during the routing back of an Ack, Nack or FloodResponse packet we would encounter an error, we would send that packet directly to destination through the Simulation controller.
This is necessary because our network doesn't have any other way to know what happens to a packet other than Acks and Nack, so they can't be lost.

### Serialization

As described in the main document, Message fragment cannot contain dynamically-sized data structures (that is, **no** `Vec`, **no** `String`, **no** `HashMap` etc.). Therefore, packets will contain large, fixed-size arrays instead.

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
	data: [u8; 128] // usable for image with .into_bytes()
}
```

To reassemble fragments into a single packet, a client or server uses the fragment header as follows:

1. The client or server receives a fragment.

2. It first checks the (`session_id`, `src_id`) tuple in the header.

3. If it has not received a fragment with the same (`session_id`, `src_id`) tuple, then it creates a vector (`Vec<u8>` with capacity of `total_n_fragments` * 128) where to copy the data of the fragments.

4. It would then copy `length` elements of the `data` array at the correct offset in the vector.

> Note: if there are more than one fragment, `length` must be 128 for all fragments except for the last. The `length` of the last one is specified by the `length` component inside the fragment,

If the client or server has already received a fragment with the same `session_id`, then it just needs to copy the data of the fragment in the vector.

Once that the client or server has received all fragments (that is, `fragment_index` 0 to `total_n_fragments` - 1), then it has reassembled the whole message and sends back an Ack.

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
    

6. If in any of the cases in which we found an error, the Packet was an Ack, Nack or FloodResponse, it'll be sent back to the destination through the Simulation Controller.


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


# Simulation Controller

Like nodes, the **Simulation Controller** (SC) runs on a thread. It must retain a means of communication with all nodes of the network.
The Simulation controller can send and receive different commands/events to/from the drones through reserved channels.
The list of available **commands** is as follows:

```rust
/// From controller to drone
pub enum DroneCommand {
    RemoveSender(NodeId),
    AddSender(NodeId, Sender<Packet>),
    SetPacketDropRate(f32),
    Crash,
}

/// From drone to controller
pub enum DroneEvent {
    PacketSent(Packet),
    PacketDropped(Packet),
    ControllerShortcut(Packet),
}
```

The Simulation Controller can execute the following tasks:

`Spawn`: This command adds a new drone to the network.

### Simulation commands

The Simulation Controller can send the following commands to drones:

`Crash`: This command makes a drone crash.
1. The Simulation Controller, while sending this command to the drone, will send also a 'RemoveSender' command to its neighbours, so that the crushing drone will be able to process the remaining messages without any other incoming.
2. At the same time the Crash command will be sent to the drone, which will put it in 'Crashing behavior'. In this state the drone will call the 'recv()' function only on its 'Receiver<Packet>' channel, process the remaining messages as follows, then when all the sender to it's channel will be removed, and the channel will be emptied, trying to listen to it will give back an error, which will mean that the drone can finally crash.
3. While in this state, the drone will process the remaining messages as follows:
- FloodRequest can be lost during the process.
- Ack, Nack and FloodResponse should still be forwarded to the next hop.
- Other types of packets will send an 'ErrorInRouting' Nack back, since the drone has crashed.

`RemoveSender(nghb_id)`: This command close the channel with a neighbour drone.

`AddSender(dst_id, crossbeam::Sender)`: This command adds `dst_id` to the drone neighbors, with `dst_id` crossbeam::Sender.

`SetPacketDropRate(pdr)`: This command alters the pdr of a drone.

#### Note:
Commands issued by the Simulation Controller must preserve the initial network requirements:

- The network graph must remain connected.
- Each client must remain connected to at least one and at most two drones.
- Each server must remain connected to at least two drones.

It is the responsibility of the Simulation Controller to validate these conditions before executing any command.

### Simulation events

The Simulation Controller can receive the following events from drones:

`PacketSent(packet)`: This event indicates that node has sent a packet. All the informations about the `src_id`, `dst_id` and `path` are stored in the packet routing header.

`PacketDropped(packet)`: This event indicates that node has dropped a packet. All the informations about the `src_id`, `dst_id` and `path` are stored in the packet routing header.

## Shortcut for Ack, Nack and FloodResponse

Since these messages cannot be lost for the network to work, the drone will send them to the Simulator in case of an error, which will send them directly to the destination.

### Communication with hosts (clients and servers)
Since the simulation controller, clients, and servers are managed within the group, the commands between the SC and hosts may differ from those used for drones, as `PacketDropped(packet)`, `Crash`, and `SetPacketDropRate(pdr)` are commands related only to them.  
However, it is preferable (and strongly recommended) to also add `AddSender(dst_id, crossbeam::Sender)` and `PacketSent(packet)` to the host commands/events.

## Note on commands and events

Due to the importance of these messages, drones MUST prioritize handling commands from the simulation controller over messages and fragments.

This can be done by using [the select_biased! macro](https://shadow.github.io/docs/rust/crossbeam/channel/macro.select_biased.html) and putting the simulation controller channel first, as seen in the example.


# **Client-Server Protocol: High-level Messages**

These are the kinds of high-level messages that we expect can be exchanged between clients and servers.

In the following, we write Protocol messages in this form:
A -> B : name(params)
where A and B are network nodes.
In this case, a message of type `name` is sent from A to B. This message
contains parameters `params`. Some messages do not provide parameters.
Notice that these messages are not subject to the rules of fragmentation, in fact, they can exchange Strings, Vecs and other dynamically-sized types

### Webserver Messages
- C -> S : server_type?
- S -> C : server_type!(type)
- C -> S : files_list?
- S -> C : files_list!(list_of_file_ids)
- C -> S : file?(file_id)
- S -> C : file!(file_size, file)
- C -> S : media?(media_id)
- S -> C : media!(media)
- S -> C : error_requested_not_found!
- S -> C : error_unsupported_request!
### Chat Messages
- C -> S : registration_to_chat,
- C -> S : client_list?
- S -> C : client_list!(list_of_client_ids)
- C -> S : message_for?(client_id, message)
- S -> C : message_from!(client_id, message)
- S -> C : error_wrong_client_id!

This is just an example. You can implement the communication as you prefer as long as it's in line with the main protocol.
