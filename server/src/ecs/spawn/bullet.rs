use std::time::{SystemTime};

use common::ecs::components::{Position, Bullet, LookDirection};
use hecs::Entity;
use common::ecs::components::{Velocity};

use crate::ecs::ServerEcs;

pub fn spawn_bullet(ecs: &mut ServerEcs, pos: Position, look_dir: LookDirection, range: f32, despawn: SystemTime) -> Entity {
    let entity = ecs.world.reserve_entity();

    // Insert observed components
    ecs.observed_world().insert(entity, (
        Bullet {
            despawn_time: despawn,
        },
        Position(pos.0),
        Velocity(look_dir.0 * range),
    )).unwrap();

    return entity;
}