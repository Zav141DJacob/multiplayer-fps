use crate::game::ecs::ClientEcs;

mod physics;
pub mod animation;
mod client_init;

/// Client-side systems are implemented onto this
pub struct ClientSystems;

impl ClientSystems {
    pub fn run(ecs: &mut ClientEcs, dt: f32) {
        ClientSystems::apply_velocity(ecs, dt);
        ClientSystems::client_init(ecs, dt);
        ClientSystems::animate_running(ecs, dt);
        ClientSystems::apply_animations(ecs, dt);
    }
}