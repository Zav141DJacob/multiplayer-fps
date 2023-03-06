use std::time::Duration;

use common::ecs::components::{Position, Bullet, LookDirection};
use hecs::Entity;
use common::ecs::components::{Velocity};
use common::ecs::timer::Timer;
use crate::ecs::components::BulletDespawn;

use crate::ecs::ServerEcs;

pub fn spawn_bullet(ecs: &mut ServerEcs, pos: Position, look_dir: LookDirection, range: f32, duration: Duration) -> Entity {
    let entity = ecs.world.reserve_entity();

    // Insert observed components
    ecs.observed_world().insert(entity, (
        Bullet,
        Position(pos.0),
        Velocity(look_dir.0 * range),
    )).unwrap();

    // Insert server-side components
    ecs.world.insert(entity, (
        Timer::new(duration, BulletDespawn),
    )).unwrap();

    entity
}