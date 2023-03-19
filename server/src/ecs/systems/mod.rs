use crate::ecs::ServerEcs;

mod physics;
mod input;
mod shoot;
mod pick_up;
mod collisions;
mod respawn;
mod ammo;

/// Server-side systems are implemented onto this
pub struct ServerSystems;

impl ServerSystems {
    pub fn run(ecs: &mut ServerEcs, dt: f32) {
        ServerSystems::input_system(ecs, dt);
        ServerSystems::move_system(ecs, dt);
        ServerSystems::shoot_system(ecs, dt);
        ServerSystems::shoot_cooldown_system(ecs, dt);
        ServerSystems::bullet_despawn_system(ecs, dt);
        ServerSystems::pick_up_system(ecs, dt);
        ServerSystems::respawn_system(ecs, dt);
        ServerSystems::collision_system(ecs, dt);
        ServerSystems::reset_to_pistol(ecs, dt);
    }
}
