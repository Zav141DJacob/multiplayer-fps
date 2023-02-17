use common::ecs::components::{Position, Velocity};
use crate::game::ecs::ClientEcs;
use crate::game::ecs::systems::ClientSystems;

impl ClientSystems {
    /// Move all entities with a position and velocity
    pub fn apply_velocity(ecs: &mut ClientEcs, dt: f32) {
        ecs.world.query_mut::<(&Velocity, &mut Position)>()
            .into_iter()
            .for_each(|(entity, (vel, pos))| {
                // Unwrap component inner types
                let pos = &mut pos.0;
                let vel = vel.0;

                // Apply velocity scaled with delta time
                *pos += vel * dt
            })
    }
}