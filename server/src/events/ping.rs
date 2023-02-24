use common::FromServerMessage;
use message_io::{network::Endpoint, node::NodeHandler};

pub fn execute(handler: &NodeHandler<()>, endpoint: Endpoint) {
    println!("Ping from {}", endpoint.addr());

    // TODO: handle errors better
    FromServerMessage::Pong
        .construct()
        .unwrap()
        .send(handler, endpoint);
}
