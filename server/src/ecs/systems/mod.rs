use crate::ecs::ServerEcs;

mod physics;
mod input;
mod shoot;
mod collisions;

/// Server-side systems are implemented onto this
pub struct ServerSystems;

impl ServerSystems {
    pub fn run(ecs: &mut ServerEcs, dt: f32) {
        ServerSystems::input_system(ecs, dt);
        ServerSystems::move_system(ecs, dt);
        // ServerSystems::shoot_system(ecs, dt);
        ServerSystems::collision_system(ecs, dt);
    }
}
