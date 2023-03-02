use common::{FromServerMessage, Signal};
use message_io::{network::Endpoint, node::NodeHandler};

pub fn execute(handler: &NodeHandler<Signal>, endpoint: Endpoint) {
    // println!("Ping from {}", endpoint.addr());

    // TODO: handle errors better
    FromServerMessage::Pong
        .construct()
        .unwrap()
        .send(handler, endpoint);
}
