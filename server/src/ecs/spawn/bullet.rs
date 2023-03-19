use std::time::Duration;
use glam::Vec2;

use common::ecs::components::{Position, Bullet, LookDirection, Player};
use rand::{Rng, thread_rng};
use common::ecs::components::{Velocity};
use common::ecs::timer::Timer;
use common::gun::Gun;
use crate::ecs::components::{BulletDespawn};

use crate::ecs::ServerEcs;

pub fn spawn_bullet(ecs: &mut ServerEcs, player: Player, pos: Position, dir: LookDirection, gun: Gun) {
    assert!(dir.0.is_normalized());
    let mut rng = thread_rng();

    for _ in 0..gun.pellets() {
        let entity = ecs.world.reserve_entity();

        let mut dir = dir.0;
        if let Some(angle) = gun.spread() {
            let angle = rng.gen_range(-angle..angle);
            dir = Vec2::from_angle(angle).rotate(dir);
        }

        // Insert observed components
        ecs.observed_world().insert(entity, (
            Bullet::new(player.id, gun),
            Position(pos.0),
            Velocity(dir * gun.bullet_speed()),
        )).unwrap();

        // Insert server-side components
        ecs.world.insert(entity, (
            Timer::new(Duration::from_secs_f32(gun.range() / gun.bullet_speed()), BulletDespawn),
        )).unwrap();
    }
}
