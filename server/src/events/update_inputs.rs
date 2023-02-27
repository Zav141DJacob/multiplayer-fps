use common::{FromServerMessage, ecs::components::InputState};
use message_io::{network::Endpoint, node::NodeHandler};

pub fn execute(handler: &NodeHandler<()>, endpoint: Endpoint, input_state: InputState, requester_id: u64) {
    println!("Ping from {}", endpoint.addr());
    


    // TODO: handle errors better

}
