use wg_2024::network::SourceRoutingHeader;
use wg_2024::packet::{FloodRequest, Fragment, NodeType, Packet};

fn main() {
    println!("----- SourceRoutingHeader -----");
    let mut route = SourceRoutingHeader::initialize(vec![1, 2, 3]);
    println!("route: {}", route);
    route.increase_hop_index();
    println!("route: {}", route);
    println!("current_hop: {:?}", route.current_hop());
    route.increase_hop_index();
    println!("route: {}", route);
    println!("next_hop: {:?}", route.next_hop());
    println!("is_last_hop: {}", route.is_last_hop());
    println!("get_reversed: {}", route.get_reversed());
    route.reset_hop_index();
    println!("reset route: {}", route);
    println!(
        "sub_route(1..): {}",
        route
            .sub_route(1..)
            .map(|r| r.to_string())
            .unwrap_or("None".to_string())
    );
    println!(
        "sub_route(2..0): {}",
        route
            .sub_route(2..0)
            .map(|r| r.to_string())
            .unwrap_or("None".to_string())
    );
    println!(
        "[ 0 -> 1 -> 2 -> 3 -> 1 -> (4) ].without_loops(): {}",
        SourceRoutingHeader::new(vec![0, 1, 2, 3, 1, 4], 0).without_loops()
    );

    println!("----- Fragment -----");
    let packet = Packet::new_fragment(
        route.clone(),
        1,
        Fragment::from_string(0, 1, "Hello, World!".to_string()),
    );
    println!("small packet: {}", packet);
    let packet = Packet::new_fragment(
        route.clone(),
        1,
        Fragment::from_string(
            0,
            1,
            "Nell mezzo del cammin di nostra vita, mi ritrovai per una selva oscura".to_string(),
        ),
    );
    println!("big packet: {}", packet);

    println!("----- Flood -----");
    let flood_request = FloodRequest::new(1, 1);
    println!("flood_request: {}", flood_request);
    let incremented = flood_request.get_incremented(2, NodeType::Client);
    println!("incremented: {}", incremented);
    let response = incremented.generate_response(1);
    println!("response: {}", response);
}
