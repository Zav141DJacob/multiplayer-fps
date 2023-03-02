use std::collections::HashMap;

use crate::ecs::ServerEcs;
use crate::{ecs::systems::ServerSystems, events};
use common::ecs::components::{Input, InputState, Player};
use common::Direction;

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
        ecs.world
            .query::<(&Player, &Input)>()
            .into_iter()
            .map(|(_, (player, input_state))| (player.id, input_state.0))
            .collect()
    }
}
