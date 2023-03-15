use std::time::Duration;

use common::ecs::components::{Position, Bullet, LookDirection, Player, Damage};
use hecs::Entity;
use common::ecs::components::{Velocity};
use common::ecs::timer::Timer;
use common::gun::Gun;
use crate::ecs::components::BulletDespawn;

use crate::ecs::ServerEcs;

pub fn spawn_bullet(ecs: &mut ServerEcs, player: Player, pos: Position, dir: LookDirection, gun: Gun) -> Entity {
// pub fn spawn_bullet(ecs: &mut ServerEcs, pos: Position, dir: LookDirection, gun: Gun) -> Entity {
    let entity = ecs.world.reserve_entity();
    assert!(dir.0.is_normalized());

    // Insert observed components
    ecs.observed_world().insert(entity, (
        Bullet::new(player.id),
        Damage(gun.damage()),
        Position(pos.0),
        Velocity(dir.0 * gun.bullet_speed()),
    )).unwrap();

    // Insert server-side components
    ecs.world.insert(entity, (
        Timer::new(Duration::from_secs_f32(gun.range() / gun.bullet_speed()), BulletDespawn),
    )).unwrap();

    entity
}