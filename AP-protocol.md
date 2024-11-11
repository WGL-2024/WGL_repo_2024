# [Faulty] The communication protocol specifications

This document provides the specifications of the communication protocol used by the drones, the client and the servers of the network. In the following document, drones, clients and servers are collectively referred to as **nodes**. The specifications are often broken or incomplete and you must improve over them.

This document also establishes some technical requirements of the project.

# Types used in this document
Can be useful for understanding and for not having to change the underlining type everywhere.

``` Rust
type NodeId = u64;

```


# Network Initializer

The **Network Initializer** reads a local **Network Initialization File** that encodes the network topology and the drone parameters and, accordingly, spawns the node threads and sets up the Rust channels for communicating between nodes.

> Importantly, the Network Initializer should also setup the Rust channels between the nodes and the Simulation Controller (see the Simulation Controller section).

## Network Initialization File
The **Network Initialization File** is in the `.toml` format, and structured as explained below:

### Drones
Any number of drones, each formatted as:
```TOML
[[drone]]
id = "drone_id"
connected_drone_ids = ["connected_id1", "connected_id2", "connected_id3", "..."]
pdr = "pdr"
```
- note that the `pdr` is defined between 0 and 1 (0.05 = 5%).
- note that `connected_drone_ids` cannot contain `drone_id` nor repetitions

### Clients
Any number of clients, each formatted as:
```TOML
[[client]]
id = "client_id"
connected_drone_ids = ["connected_id1", "..."] # max 2 entries
```
- note that `connected_drone_ids` cannot contain `client_id` nor repetitions
- note that a client cannot connect to other clients or servers
- note that a client can be connected to at most two drones

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

### Example
```toml
[[drone]]
id = 1
connected_drone_ids = [2, 3]
pdr = 0.05

[[drone]]
id = 2
connected_drone_ids = [1,3,4]
pdr = 0.03

[[drone]]
id = 3
connected_drone_ids = [2,1,4]
pdr = 0.14

[[client]]
id = 4
connected_drone_ids = [3, 2]

[[client]]
id = 5
connected_drone_ids = [1]

[[server]]
id = 6
connected_drone_ids = [2,3]
```

### Initialization algorithm

The TOML file just defined is parsed through the code seen in the parser.rs file, which returns a Config object.

```rust 
struct Config {
    drone: Vec<Drone>,
    client: Vec<Client>,
    server: Vec<Server>,
}
```

With this information we need to create all channels and spawn all the threads, an algorithm such as the following can be used

```rust
/// sample code
/// only drones are covered here, but clients and servers should be the same

	//store channels
    let mut packet_channels: Vec<(Sender<Packet>, Receiver<Packet>)> = Vec::new();
    //simulation controller channel
    let mut sc_channels: Vec<(Sender<SimulationControllerCommand>, Receiver<SimulationControllerCommand>)> = Vec::new();

    //create 3 different version since we might want the simulation controller channels to depend on node type
    for _ in config.drone.iter() {
        //create unbounded channel for drones
        packet_channels.push(unbounded::<Packet>());
        sc_channels.push(unbounded::<SimulationControllerCommand>());
    }

	for drone in config.drone.iter() {

        //clones all the sender channels for the connected drones
        let sender_channels: Vec<(u64, Sender<Packet>)> = drone.connected_drone_ids.iter().map(|&x| (x, packet_channels[x as usize].0.clone())).collect();

        let packet_receiver = packet_channels[drone.id as usize].1.clone();
        let command_receiver = sc_channels[drone.id as usize].1.clone();

		// since the thread::spawn function will take ownership of the values, we need to copy or clone them to not have problems with the Vec
        let drone_id = drone.id;

        let pdr = drone.pdr;

        thread::spawn(move || {

            let mut neighbors_init: HashMap<u64, Sender<Packet>> = HashMap::new();

            for (id, sender) in sender_channels.iter() {
                neighbors_init.insert(*id, sender.clone());
            }

            let mut drone = Drone {
                floods_tracker: HashMap::new(),
                neighbors: neighbors_init,
                id: drone_id,
                packet_rec: packet_receiver,
                sim_controller_rec: command_receiver,
                pdr: pdr,
            };

            //run function is where the logic of the drone runs.
            drone.run();
        });
    }
```

### Packet Handling

Inside each node the the two receiving channels need to be handled at the same time. This is done using crossbeam's select! macro.
Here's an example of how this would look inside the Drone::run() function

```rust
/// sample code

	fn run(&mut self) {
		//runs forever
		loop {
			select! {
				recv(self.packet_rec) -> packet_res => {
					if let Ok(packet) = packet_res {
						// each match branch may call a function to handle it to make it more readable
						match packet.pack_type {
							PacketType::Nack(nack) => todo!(),
							PacketType::Query(query) => todo!(),
							PacketType::QueryResult(query_result) => todo!(),
						}
					}
				},
				recv(self.sim_controller_rec) -> command_res => {
					if let Ok(command) = command_res {
						//handle the simulation controller's command
					}
				}
			}
		}
	}
```


# Drone parameters: Packet Drop Rate

A drone is characterized by a parameter that regulates what to do when a packet is received, that thus influences the simulation. This parameter is provided in the Network Initialization File.

Packet Drop Rate: The drone drops the received packet with probability equal to the Packet Drop Rate.

# Messages and fragments

Recall that there are: Content servers (that is, Text and Media servers) and Communication servers. These servers are used by clients to implement applications.

These servers exchange, respectively, Text server messages, Media server messages and Communication server messages. These are high-level messages. Recall that you must standardize and regulate their low-level counterparts (that is, fragments).

# Source routing

The fragments that circulate in the network are **source-routed** (except for the commands sent from and the events received by the Simulation Controller).

Source routing refers to a technique where the sender of a data packet specifies the route the packet takes through the network. This is in contrast with conventional routing, where routers in the network determine the path incrementally based on the packet's destination.

The consequence is that drones do not need to maintain routing tables.

As an example, consider the following simplified network:

![constellation](costellation.png)

Suppose that the client A wants to send a message to the server D.

It computes the route B→E→F→D, creates a **Source Routing Header** specifying route A→B→E→F→D, adds it to the packet and sends it to B.

When B receives the packet, it sees that the next hop is E and sends the packet to it.

When E receives the packet, it sees that the next hop is F and sends the packet to it.

When F receives the packet, it sees that the next hop is D and sends the packet to it.

When D receives the packet, it sees there are no more hops so it must be the final destination: it can thus process the packet.

```rust
struct SourceRoutingHeader {
	/// Vector of nodes with initiator and nodes to which the packet will be forwarded to.
	hops: Vec<u64>
}
```

## Network **Discovery Protocol**

When the network is first initialized, nodes only know who their own neighbours are.

Clients and servers need to obtain an understanding of the network topology (”what nodes are there in the network and what are their types?”) so that they can compute a route that packets take through the network (refer to the Source routing section for details).

To do so, they must use the **Network Discovery Protocol**. The Network Discovery Protocol is initiated by clients and servers and works through query flooding.

### **Flooding Initialization**

The client or server that wants to learn the topology, called the **initiator**, starts by flooding a query to all its immediate neighbors:

<mark> Discussion point: Should queries be fragments/packets? If yes then source_routing_header should be removed and both Query and QueryResult needs to be added to the Packet struct. </br>
Also reminder that Query </mark>

```rust
enum NodeType{Client, Drone, Server}

struct Query {
	/// Unique identifier of the flood, to prevent loops.
	flood_id: u64,
	/// ID of client or server
	initiator_id: u64,
	/// Time To Live, decremented at each hop to limit the query's lifespan.
	/// When ttl reaches 0, we start a QueryResult message that reaches back to the initiator
	ttl: u64,
	/// Records the nodes that have been traversed (to track the connections).
	path_trace: Vec<(u64, NodeType)>,
	/// Broadcasting query, this means that no QueryResult needs to be sent back
	broadcasting: bool
}

struct QueryResult {
	/// Unique indentifier of the flood, this allows the initiator to identify the information obtained by the latest flood only
	flood_id: u64,
	/// Record of the nodes traversed by the flooding query
	path_trace: Vec<(u64, NodeType)>
}
```

### **Neighbor Response**

Each node stores a hashmap which associates flood_ids to node_ids. This assumes flood_ids are increased incrementally rather than chosen arbitrarily. 
<mark> flood_id 0 is also reserved for easier computation </mark>
```rust
//sample code
floods_tracker: HashMap<u64, u64>
// HashMap<K: initiator_id, V: flood_id>
```

When a neighbor node receives the query, it processes it based on the following rules:

- checks the floods_tracker HashMap for the value stored for the initiator_id. 
- If there's no value or the value is lower than the flood_id, it means that the current flood is the latest one, so it should be processed accordingly
	- Processing entails adding the information of the current node into the path_trace, decreasing the TTL by one, and forwarding the query to all neighbors except for the one it received the query from
- If the value stored is the same as the flood_id, it means the current node has already processed the current flood, so it should be processed as follows
	- Build a QueryResult with the path_trace information, the path_trace from the initiator_id to the *first* instance of the current node can be reversed and used as the hops inside source_routing_header
- If the TTL of the message is 0, add the current node's information to the path_trace, then process it like the step right above this one.

```rust
//sample code

	// packet: Packet
	// the packet.packet_type was matched for PacketType::Query
	// query: Query


	// push node into path_trace, this assures that every edge can be reconstructed by the initiator node
	query.path_trace.push((self.id, NodeType::Drone));

	if (query.ttl == 0) {

		//only send back a result if the flooding is not a broadcast
		if !query.broadcasting {
			let query_result = QueryResult {
				path_trace: query.path_trace.clone(),
				flood_id: query.flood_id
			};

			let return_routing: Vec<u64> = query.path_trace.iter().rev().map(|(node_id, _node_type)| *node_id).collect();

			let packet = Packet {
				pack_type: PacketType::QueryResult(query_result),
				routing_header: SourceRoutingHeader {
					hops: return_routing.clone()
				},
				session_id: 0, // it'll be whatever for now
			};

			let first_hop_channel = self.neighbors.get(&return_routing[0]).unwrap(); 
			// unwrap since given the construction of the hops we're guaranteed to not have any errors

			let _ = first_hop_channel.send(packet);
		}

	} else {

		//check if the flood was already managed on this node

		let floodVal = self.floods_tracker.get(&query.initiator_id).or(Some(&0));

		if let Some(&flood_id) = floodVal {
			if flood_id < query.flood_id {
				// this means that the query's flood id is a new flood, thus it needs to be processed

				query.ttl -= 1;

				for (neighbor_id, neighbor_channel) in &self.neighbors {
	
					let mut route = packet.routing_header.hops.clone();
	
					route.push(*neighbor_id);
	
					let packet = Packet {
						pack_type: PacketType::Query(query.clone()),
						routing_header: SourceRoutingHeader {
							hops: route
						},
						session_id: 0, // it'll be whatever for now
					};
				}

			} else {
				//flood's already passed through this node, thus send back a QueryResult

				
				//only send back a result if the flooding is not a broadcast
				if !query.broadcasting {
					let query_result = QueryResult {
						path_trace: query.path_trace.clone(),
						flood_id: query.flood_id
					};
		
					let mut return_routing: Vec<u64> = query.path_trace.iter().map_while(|(node_id, _node_type)| 
					if *node_id != self.id {
						Some(*node_id)
					} else {
						None
					}).collect();

					return_routing.reverse();
		
					let packet = Packet {
						pack_type: PacketType::QueryResult(query_result),
						routing_header: SourceRoutingHeader {
							hops: return_routing.clone()
						},
						session_id: 0, // it'll be whatever for now
					};
		
					let first_hop_channel = self.neighbors.get(&return_routing[0]).unwrap(); 
					// unwrap since given the construction of the hops we're guaranteed to not have any errors
		
					let _ = first_hop_channel.send(packet);
				}
			}
		}
	}

```

### **Recording Topology Information**

When the initiator receives a QueryResult, it will update its understanding of the topology through an algorithm.

```rust
/// sample code 
	// topology: Graph { nodes: HashSet<(u64, NodeType)>, edges: HashSet<(u64, u64)> }
    // path_trace: Vec<(u64, NodeType)>

    //by design the first element always exists
    if path_trace.pop().unwrap().0 == id {

        let mut current_node = id;

        for (node_id, node_type) in path_trace {

            // since we're using a HashSet for both Nodes and Edges, we can insert without worrying about the nodes already existing
            topology.add_node(*node_id, *node_type);
            topology.add_edge(current_node, *node_id);

            current_node = *node_id;
        }
    }
```

Neighbors will process this struct like a query struct, 

### **Termination Condition**

The flood automatically ends once all nodes have propagated the message, or when all Queries have reached their TTL.

<mark> When, apart from initialization, should a flood be run? Periodically or only if a node is unreachable because the topology hasn't had the chance to updated properly yet? Discussion point </mark>

### **Dealing with crashed drones**

When a client receives an Error packet, it is implied that the drone has crashed, thus we can remove it from the client's topology

```rust
// sample code
	// packet: Packet
    // packet.PacketType matched to Nack(crashed)
    // nack: Nack

    if let NackType::ErrorInRouting(crashed_node_id) = nack.nack_type {
        topology.remove_drone(crashed_node_id);

        for neighbor_id in topology.adj(crashed_node_id) {
            topology.remove_edge(crashed_node_id, neighbor_id);
        }

    }
```

### **Broadcasting new drone spawning**

When the simulation controller adds a new drone, it needs to broadcast its own existance to all the other drones. 
This is done through flooding a query with the broadcast property set to true.
Additionally we add an extra piece of logic in the client and server nodes to properly process it

```rust
//sample code

	// packet: Packet
    // the packet.packet_type was matched for PacketType::Query
    // query: Query

    let mut path_trace = query.path_trace;
    path_trace.reverse();

    let mut current_node = id;

    for (node_id, node_type) in path_trace {
        // since we're using a HashSet for both Nodes and Edges, we can insert without worrying about the nodes already existing
        topology.add_node(node_id, node_type);
        topology.add_edge(current_node, node_id);

        current_node = node_id;
    }

```


# **Client-Server Protocol: Fragments**

Clients and servers exchange packets that are routed through the drone network. The Client-Server Protocol standardizes and regulates the format of these packets and their exchange.

These packets can be: Message, Ack, Nack, Query, QueryResult.

As described in the main document, Message packets must be serialized and can be possibly fragmented, and the fragments can be possibly dropped by drones.

```
						META-LEVEL COMMENT
		This section is clearly underspecified, each message
		should be a struct, with a certain name, and perhaps
		a fixed API that can be called upon that struct.
		The WG must also define that API and implement it:
		that means writing some shared code that all groups
		will download and execute in order to manage packets.
```

### Message

Message is subject to fragmentation: see the dedicated section.

Message (and Message only) can be dropped by drones.

```rust
#[derive(Debug)]
pub enum ServerType{
	ChatServer, // only does chat
	TextServer, // only does text
	MediaServer, // does text and media
	HybridServer // does all of them
}

#[derive(Debug)]
pub struct Message{
	message_data: MessageData,
	routing_header: SourceRoutingHeader
}

#[derive(Debug)]
pub struct MessageData { // Only part fragmentized
	session_id: u64,
	content: MessageContent
}
```

#### Message Types
```rust
#[derive(Debug)]
pub enum MessageContent{
	// Client -> Server
	ReqServerType,
	ReqFilesList,
	ReqFile(u64),
	ReqMedia(u64),

	ReqClientList,
	ReqMessageSend { to: NodeId, message: Vec<u8> },
	// Do we need request of new messages? or directly sent by server?

	// Server -> Client
	RespServerType(ServerType)
	RespFilesList(Vec<u64>),
	RespFile(u64, Vec<u8>),
	RespMedia(u64, Vec<u8>),
	ErrUnsupporedRequestType,
	ErrRequestedNotFound

	RespClientList(Vec<NodeId>),
	RespMessageFrom { from: NodeId, message: Vec<u8> },
	ErrWrongClientId,
}
```

Example of new request:
```rust
let routing = getRoutingHeader();
let content = MessageType:ReqFile(8);
Message::new(routing, source_id, session_id, content)
```


### NACK
If an error occurs that a NACK is sent. A NACK can be of type:
1. **ErrorInRouting**: If a drone receives a Message and the next hop specified in the Source Routing Header is not a neighbor of the drone, then it sends Error to the client.
2. **Dropped**: If a drone receives a Message that must be dropped due to the Packet Drop Probability, then it sends Dropped to the client.

Source Routing Header contains the path to the client, which can be obtained by reversing the list of hops contained in the Source Routing Header of the problematic Message.

This message cannot be dropped by drones due to Packet Drop Probability.

```rust
pub struct Nack{
	fragment_index: u64,
	time_of_fail: std::time::Instant,
	nack_type: NackType
}

pub enum NackType{
	ErrorInRouting(NodeId), // contains id of not neighbor
	Dropped()
}
```

### Ack

<mark> What is the point of using ACKs? If a packet is dropped the client gets notified, and the same happens if the drone crashes, making ACKs fundamentally useless. Consider removing them or finding an actual usage </mark>

If a drone receives a Message and can forward it to the next hop, it also sends an Ack to the client.

```rust
pub struct Ack{
	fragment_index: u64,
	time_received: std::time::Instant
}
```

Source Routing Header contains the path to the client, which can be obtained by reversing the list of hops contained in the Source Routing Header of the problematic Message.

### Serialization

As described in the main document, messages cannot contain dynamically-sized data structures (that is, **no** `Vec`, **no** `String`, etc). Therefore, packets will contain large, fixed-size arrays instead. (Note that it would be definitely possible to send `Vec`s and `String`s through Rust channels).

In particular, Message packets have the following **limitation**:
- Arrays of up to 80 `u8` (aka bytes), which are used to communicate the `String`s that
constitute files.

### Fragment reassembly

<mark> 
Before the change there was no sequence_number. Whilst fragment_index and total_n_fragments inside the MsgFragment(Fragment) allowed to reconstruct a single message, if we had different messages arriving at the same time (if we want to allow that instead of doing 1 to 1 communication), we wouldn't know which way to assemble the fragments in. <br>
Imagine a server having to send 2 pictures of the same size (so same total_n_fragments) to a client (same session_id). <br>
How do we know which of the responses the 2/100 fragment_index belongs to? <br>
Using a sequence_number (or any number for a similar function), would fix this issue. <br>
</mark>


```rust
//fragment defined as atomic message exchanged by the drones.
pub struct Packet {
	pack_type: PacketType,
	routing_header: SourceRoutingHeader,
	session_id: u64,
}

pub enum PacketType {
	MsgFragment(Fragment),
	Nack(Nack),
	Ack(Ack),
	Query(Query),
	QueryResult(QueryResult),
}

// fragment defined as part of a message.
pub struct Fragment {
	fragment_index: u64,
	total_n_fragments: u64,
	sequence_number: u128,
	length: u8,
	// assembler will fragment/defragment data into bytes.
	data: [u8; 80] // usable for image with .into_bytes()
}
```

To reassemble fragments into a single packet, a client or server uses the fragment header as follows.

The client or server receives a fragment.

It first checks the `session_id` in the header.

If it has not received a fragment with the same `session_id`, then it creates a vector big enough where to copy the data of the fragments. Consider, for example, the `FileResponse` fragment:

```rust
FileResponse {
	file_size: u64,
	file: [char; 20],
}
```

The client would need to create a vector of type `char` with capacity of `total_n_fragments` * 20.

It would then copy `file_size` elements of the `file` array at the correct offset in the vector.

Note that, if there are more than one fragment, `file_size` is 20 for all fragments except for the last, as the arrays carry as much data as possible.

If the client or server has already received a fragment with the same `session_id`, then it just needs to copy the data of the fragment in the vector.

Once that the client or server has received all fragments (that is, `fragment_index` 0 to `total_n_fragments` -2), then it has reassembled the whole fragment.

Therefore, the packet is now a message that can be delivered.

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

```
										META-LEVEL COMMENT
		This section is clearly underspecified: what is the
		type of `metadata`? It is your duty as WG to define
		these things.
```

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

- C -> S : client_list?
- S -> C : client_list!(list_of_client_ids)
- C -> S : message_for?(client_id, message)
- S -> C : message_from!(client_id, message)
- S -> C : error_wrong_client_id!
