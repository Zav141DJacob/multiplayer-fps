use common::{ecs::components::{InputState, Player, Input}};

use crate::server::Server;

pub fn execute(server: &mut Server, updated_input_state: InputState, requester_id: u64) {
    
    let (entity, (_, input_state)) = server
            .ecs
            .world
            .query_mut::<(&Player, &mut Input)>()
            .into_iter()
            .find(|(_, (&player, _))| {
                player.id == requester_id
            })
            .unwrap();

    *server.ecs.observer.observe_component(entity, input_state) = Input(updated_input_state);
    server.ecs.observer.drain_reliable();


    // TODO: handle errors better

}
