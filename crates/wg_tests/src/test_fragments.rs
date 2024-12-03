use crossbeam_channel::unbounded;
use std::collections::HashMap;
use std::thread;
use wg_drone::Drone;
use wg_network::SourceRoutingHeader;
use wg_packet::{Ack, Fragment, Nack, NackType, Packet, PacketType};

/* THE FOLLOWING TESTS CHECKS IF YOUR DRONE IS HANDLING CORRECTLY PACKETS (FRAGMENT) */

/// Creates a sample packet for testing purposes. For convenience, using 1-10 for clients, 11-20 for drones and 21-30 for servers
fn create_sample_packet() -> Packet {
    Packet {
        pack_type: PacketType::MsgFragment(Fragment {
            fragment_index: 1,
            total_n_fragments: 1,
            length: 128,
            data: [1; 128],
        }),
        routing_header: SourceRoutingHeader {
            hop_index: 1,
            hops: vec![1, 11, 12, 21],
        },
        session_id: 1,
    }
}

/// This function is used to test the packet forward functionality of a drone.
pub fn generic_fragment_forward<T: Drone + Send + 'static>() {
    // drone 2 <Packet>
    let (d_send, d_recv) = unbounded();
    // drone 3 <Packet>
    let (d2_send, d2_recv) = unbounded::<Packet>();
    // SC commands
    let (_d_command_send, d_command_recv) = unbounded();

    let neighbours = HashMap::from([(12, d2_send.clone())]);
    let mut drone = T::new(
        11,
        unbounded().0,
        d_command_recv,
        d_recv.clone(),
        neighbours,
        0.0,
    );
    // Spawn the drone's run method in a separate thread
    thread::spawn(move || {
        drone.run();
    });

    let mut msg = create_sample_packet();

    // "Client" sends packet to d
    d_send.send(msg.clone()).unwrap();
    msg.routing_header.hop_index = 2;

    // d2 receives packet from d1
    assert_eq!(d2_recv.recv().unwrap(), msg);
}

/// Checks if the packet is dropped by one drone. The drone MUST have 100% packet drop rate, otherwise the test will fail sometimes.
pub fn generic_fragment_drop<T: Drone + Send + 'static>() {
    // Client 1
    let (c_send, c_recv) = unbounded();
    // Drone 11
    let (d_send, d_recv) = unbounded();
    // SC commands
    let (_d_command_send, d_command_recv) = unbounded();

    let neighbours = HashMap::from([(12, d_send.clone()), (1, c_send.clone())]);
    let mut drone = T::new(
        11,
        unbounded().0,
        d_command_recv,
        d_recv.clone(),
        neighbours,
        1.0,
    );

    // Spawn the drone's run method in a separate thread
    thread::spawn(move || {
        drone.run();
    });

    let msg = create_sample_packet();

    // "Client" sends packet to the drone
    d_send.send(msg.clone()).unwrap();

    let dropped = Nack {
        fragment_index: 1,
        nack_type: NackType::Dropped,
    };
    let srh = SourceRoutingHeader {
        hop_index: 1,
        hops: vec![11, 1],
    };
    let nack_packet = Packet {
        pack_type: PacketType::Nack(dropped),
        routing_header: srh,
        session_id: 1,
    };

    // Client listens for packet from the drone (Dropped Nack)
    assert_eq!(c_recv.recv().unwrap(), nack_packet);
}

/// Checks if the packet is dropped by the second drone. The first drone must have 0% PDR and the second one 100% PDR, otherwise the test will fail sometimes.
pub fn generic_chain_fragment_drop<T: Drone + Send + 'static>() {
    // Client 1 channels
    let (c_send, c_recv) = unbounded();
    // Server 21 channels
    let (s_send, _s_recv) = unbounded();
    // Drone 11
    let (d_send, d_recv) = unbounded();
    // Drone 12
    let (d12_send, d12_recv) = unbounded();
    // SC - needed to not make the drone crash
    let (_d_command_send, d_command_recv) = unbounded();

    // Drone 11
    let neighbours11 = HashMap::from([(12, d12_send.clone()), (1, c_send.clone())]);
    let mut drone = T::new(
        11,
        unbounded().0,
        d_command_recv.clone(),
        d_recv.clone(),
        neighbours11,
        0.0,
    );
    // Drone 12
    let neighbours12 = HashMap::from([(11, d_send.clone()), (21, s_send.clone())]);
    let mut drone2 = T::new(
        12,
        unbounded().0,
        d_command_recv.clone(),
        d12_recv.clone(),
        neighbours12,
        1.0,
    );

    // Spawn the drone's run method in a separate thread
    thread::spawn(move || {
        drone.run();
    });

    thread::spawn(move || {
        drone2.run();
    });

    let msg = create_sample_packet();

    // "Client" sends packet to the drone
    d_send.send(msg.clone()).unwrap();

    // Client receive an ACK originated from 'd'
    assert_eq!(
        c_recv.recv().unwrap(),
        Packet {
            pack_type: PacketType::Ack(Ack { fragment_index: 1 }),
            routing_header: SourceRoutingHeader {
                hop_index: 1,
                hops: vec![11, 1],
            },
            session_id: 1,
        }
    );

    // Client receive an NACK originated from 'd2'
    assert_eq!(
        c_recv.recv().unwrap(),
        Packet {
            pack_type: PacketType::Nack(Nack {
                fragment_index: 1,
                nack_type: NackType::Dropped,
            }),
            routing_header: SourceRoutingHeader {
                hop_index: 2,
                hops: vec![12, 11, 1],
            },
            session_id: 1,
        }
    );
}

/// Checks if the packet can reach its destination. Both drones must have 0% PDR, otherwise the test will fail sometimes.
pub fn generic_chain_fragment_ack<T: Drone + Send + 'static>() {
    // Client<1> channels
    let (c_send, c_recv) = unbounded();
    // Server<21> channels
    let (s_send, s_recv) = unbounded();
    // Drone 11
    let (d_send, d_recv) = unbounded();
    // Drone 12
    let (d12_send, d12_recv) = unbounded();
    // SC - needed to not make the drone crash
    let (_d_command_send, d_command_recv) = unbounded();

    // Drone 11
    let neighbours11 = HashMap::from([(12, d12_send.clone()), (1, c_send.clone())]);
    let mut drone = T::new(
        11,
        unbounded().0,
        d_command_recv.clone(),
        d_recv.clone(),
        neighbours11,
        0.0,
    );
    // Drone 12
    let neighbours12 = HashMap::from([(11, d_send.clone()), (21, s_send.clone())]);
    let mut drone2 = T::new(
        12,
        unbounded().0,
        d_command_recv.clone(),
        d12_recv.clone(),
        neighbours12,
        0.0,
    );

    // Spawn the drone's run method in a separate thread
    thread::spawn(move || {
        drone.run();
    });

    thread::spawn(move || {
        drone2.run();
    });

    let mut msg = create_sample_packet();

    // "Client" sends packet to the drone
    d_send.send(msg.clone()).unwrap();

    // Client receive an ACK originated from 'd'
    assert_eq!(
        c_recv.recv().unwrap(),
        Packet {
            pack_type: PacketType::Ack(Ack { fragment_index: 1 }),
            routing_header: SourceRoutingHeader {
                hop_index: 1,
                hops: vec![11, 1],
            },
            session_id: 1,
        }
    );

    // Client receive an ACK originated from 'd2'
    assert_eq!(
        c_recv.recv().unwrap(),
        Packet {
            pack_type: PacketType::Ack(Ack { fragment_index: 1 }),
            routing_header: SourceRoutingHeader {
                hop_index: 2,
                hops: vec![12, 11, 1],
            },
            session_id: 1,
        }
    );

    msg.routing_header.hop_index = 3;
    // Server receives the fragment
    assert_eq!(s_recv.recv().unwrap(), msg);
}
