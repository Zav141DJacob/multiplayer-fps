use crate::game::ecs::ClientEcs;

mod physics;

/// Client-side systems are implemented onto this
pub struct ClientSystems;

impl ClientSystems {
    pub fn run(ecs: &mut ClientEcs, dt: f32) {
        ClientSystems::apply_velocity(ecs, dt);
    }
}