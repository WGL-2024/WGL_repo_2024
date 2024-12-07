use crossbeam_channel::unbounded;
use std::collections::HashMap;
use std::thread;
use wg_controller::DroneEvent;
use wg_drone::Drone;
use wg_network::SourceRoutingHeader;
use wg_packet::{Fragment, Nack, NackType, Packet, PacketType};

/* THE FOLLOWING TESTS ARE TO CHECK IF YOUR DRONE IS HANDLING CORRECTLY PACKETS (FRAGMENT) */

/// Creates a sample packet for testing purposes. For convenience, using 1-10 for clients, 11-20 for drones and 21-30 for servers
fn create_sample_packet() -> Packet {
    Packet::new_fragment(
        SourceRoutingHeader {
            hop_index: 1,
            hops: vec![1, 11, 12, 21],
        },
        1,
        Fragment {
            fragment_index: 1,
            total_n_fragments: 1,
            length: 128,
            data: [1; 128],
        },
    )
}

/// This function is used to test the packet forward functionality of a drone.
/// The assert consists in checking if the "client" and "SC" receive the correct packet.
pub fn generic_fragment_forward<T: Drone + Send + 'static>() {
    // Drone 11
    let (d_send, d_recv) = unbounded();
    // Drone 12
    let (d2_send, d2_recv) = unbounded::<Packet>();
    // SC commands
    let (_d_command_send, d_command_recv) = unbounded();
    let (d_event_send, d_event_recv) = unbounded();

    let mut drone = T::new(
        11,
        d_event_send,
        d_command_recv,
        d_recv.clone(),
        HashMap::from([(12, d2_send.clone())]),
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
    // SC listen for event from the drone
    assert_eq!(d_event_recv.recv().unwrap(), DroneEvent::PacketSent(msg));
}

/// Checks if the packet is dropped by one drone. The assert consists in checking if the "client" and "SC" receive the correct packet.
pub fn generic_fragment_drop<T: Drone + Send + 'static>() {
    // Client 1
    let (c_send, c_recv) = unbounded();
    // Drone 11
    let (d_send, d_recv) = unbounded();
    // Drone 12
    let (d2_send, _d2_recv) = unbounded();
    // SC commands
    let (_d_command_send, d_command_recv) = unbounded();
    let (d_event_send, d_event_recv) = unbounded();

    let mut drone = T::new(
        11,
        d_event_send,
        d_command_recv,
        d_recv,
        HashMap::from([(12, d2_send.clone()), (1, c_send.clone())]),
        1.0,
    );

    // Spawn the drone's run method in a separate thread
    thread::spawn(move || {
        drone.run();
    });

    let msg = create_sample_packet();

    // "Client" sends packet to the drone
    d_send.send(msg.clone()).unwrap();

    let nack_packet = Packet::new_nack(
        SourceRoutingHeader {
            hop_index: 1,
            hops: vec![11, 1],
        },
        1,
        Nack {
            fragment_index: 1,
            nack_type: NackType::Dropped,
        },
    );

    // Client listens for packet from the drone (Dropped Nack)
    assert_eq!(c_recv.recv().unwrap(), nack_packet);
    // SC listen for event from the drone
    assert_eq!(d_event_recv.recv().unwrap(), DroneEvent::PacketDropped(msg));
}

/// Checks if the packet is dropped by the second drone. The first drone has 0% PDR and the second one 100% PDR, otherwise the test will fail sometimes.
/// The assert is checking only the NACK received by the client (It does not care about the SC events).
pub fn generic_chain_fragment_drop<T: Drone + Send + 'static>() {
    // Client 1 channels
    let (c_send, c_recv) = unbounded();
    // Server 21 channels
    let (s_send, _s_recv) = unbounded();
    // Drone 11
    let (d_send, d_recv) = unbounded();
    // Drone 12
    let (d12_send, d12_recv) = unbounded();
    // SC - needed to not make the drone crash / send DroneEvents
    let (_d_command_send, d_command_recv) = unbounded();
    let (d_event_send, _d_event_recv) = unbounded();

    // Drone 11
    let mut drone = T::new(
        11,
        d_event_send.clone(),
        d_command_recv.clone(),
        d_recv,
        HashMap::from([(12, d12_send.clone()), (1, c_send.clone())]),
        0.0,
    );
    // Drone 12
    let mut drone2 = T::new(
        12,
        d_event_send.clone(),
        d_command_recv.clone(),
        d12_recv,
        HashMap::from([(11, d_send.clone()), (21, s_send.clone())]),
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

    // Client receives an NACK originated from 'd2'
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
/// The assert is checking only the ACK received by the client (It does not care about the SC events).
pub fn generic_chain_fragment_ack<T: Drone + Send + 'static>() {
    // Client 1
    let (c_send, c_recv) = unbounded();
    // Server 21
    let (s_send, s_recv) = unbounded();
    // Drone 11
    let (d_send, d_recv) = unbounded();
    // Drone 12
    let (d12_send, d12_recv) = unbounded();
    // SC - needed to not make the drone crash
    let (_d_command_send, d_command_recv) = unbounded();
    let (d_event_send, _d_event_recv) = unbounded();

    // Drone 11
    let mut drone = T::new(
        11,
        d_event_send.clone(),
        d_command_recv.clone(),
        d_recv,
        HashMap::from([(12, d12_send.clone()), (1, c_send.clone())]),
        0.0,
    );
    // Drone 12
    let mut drone2 = T::new(
        12,
        d_event_send.clone(),
        d_command_recv.clone(),
        d12_recv,
        HashMap::from([(11, d_send.clone()), (21, s_send.clone())]),
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

    msg.routing_header.hop_index = 3;
    // Server receives the fragment
    assert_eq!(s_recv.recv().unwrap(), msg);

    // Server sends an ACK
    d12_send
        .send(Packet::new_ack(
            SourceRoutingHeader {
                hop_index: 1,
                hops: vec![21, 12, 11, 1],
            },
            1,
            1,
        ))
        .unwrap();

    // Client receives an ACK originated from 's'
    assert_eq!(
        c_recv.recv().unwrap(),
        Packet::new_ack(
            SourceRoutingHeader {
                hop_index: 3,
                hops: vec![21, 12, 11, 1],
            },
            1,
            1,
        )
    );
}
