use message_io::network::Endpoint;
use common::{ecs::components::InputState};

use crate::server::Server;

// TODO: handle errors better
pub fn execute(server: &mut Server, updated_input_state: InputState, endpoint: Endpoint) {
    let entity = *server.registered_clients.get(&endpoint)
        .expect("tried to update input for unregistered client");

    let input = server.ecs.world.query_one_mut::<&mut InputState>(entity)
        .expect("client is registered but not found in ecs");

    *input = updated_input_state;
}
