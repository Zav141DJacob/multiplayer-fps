use common::Direction;

use crate::ecs::ServerEcs;

mod physics;
mod input;
mod shoot;

/// Server-side systems are implemented onto this
pub struct ServerSystems;

impl ServerSystems {
    pub fn run(ecs: &mut ServerEcs, dt: f32) {
        ServerSystems::input_system(ecs, dt);
        ServerSystems::move_system(ecs, dt);

        // ServerSystems::apply_shoot(ecs, k);
    }
}
