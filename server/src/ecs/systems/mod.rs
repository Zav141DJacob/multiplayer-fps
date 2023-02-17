use crate::ecs::ServerEcs;

mod physics;

/// Server-side systems are implemented onto this
pub struct ServerSystems;

impl ServerSystems {
    pub fn run(ecs: &mut ServerEcs, dt: f32) {
        ServerSystems::apply_velocity(ecs, dt);
    }
}
