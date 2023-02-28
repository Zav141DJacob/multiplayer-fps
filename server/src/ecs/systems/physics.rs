use std::collections::HashMap;

use crate::{ecs::systems::ServerSystems, events};
use crate::ecs::ServerEcs;
use common::Direction;
use common::ecs::components::{Position, Velocity, InputState, Player, Input};

impl ServerSystems {
    /// Move all entities with a position and velocity
    pub fn apply_velocity(ecs: &mut ServerEcs, dir: Direction, requester_id: u64) {
        events::r#move::execute(ecs, dir, requester_id);
    }

    pub fn apply_turning(ecs: &mut ServerEcs, dir: Direction, requester_id: u64) {
        events::r#turn::execute(ecs, dir, requester_id);
    }

    pub fn apply_shoot(ecs: &mut ServerEcs, requester_id: u64) {
        // events::r#move::execute(ecs, dir, requester_id);
    }

    pub fn get_input_states(ecs: &ServerEcs) -> HashMap<u64, InputState> {
        let mut return_map: HashMap<u64, InputState> = HashMap::new();
        ecs.world
            .query::<(&Player, &Input)>()
            .into_iter()
            .for_each(|(entity, (player, input_state))| {
                dbg!(player);
                return_map.insert(player.id, input_state.0);
            });
        return_map
    }

    
}
