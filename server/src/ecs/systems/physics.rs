use crate::ecs::ServerEcs;
use crate::{ecs::systems::ServerSystems};
use common::ecs::components::{Position, Velocity};

impl ServerSystems {
    /// Move all entities with a position and velocity
    pub fn move_system(ecs: &mut ServerEcs, dt: f32) {
        let query = ecs.world.query_mut::<(&Velocity, &mut Position)>();

        for (entity, (vel, pos)) in query {
            // Observe the shared components we mutate
            let mut pos = ecs.observer.observe_component(entity, pos);

            // Unwrap component inner types
            let pos = &mut pos.0;
            let vel = vel.0;

            // Apply velocity scaled with delta time
            *pos += vel * dt
        }
    }
}
