use crate::ecs::spawn::bullet::spawn_bullet;
use crate::ecs::systems::ServerSystems;
use crate::ecs::ServerEcs;
use common::ecs::components::{ HeldWeapon, InputState, LookDirection, Position, Player };
use std::time::Duration;
use common::ecs::timer::Timer;
use crate::ecs::components::{BulletDespawn, ShootCooldown};

struct BulletSpawn {
    player: Player,
    pos: Position,
    dir: LookDirection,
    range: f32,
    duration: Duration,
}

impl ServerSystems {
    /// Applies input state to Velocity and LookDirection
    pub fn shoot_system(ecs: &mut ServerEcs, _dt: f32) {
        let query = ecs
            .world
            .query_mut::<(&Player, &InputState, &LookDirection, &Position, &HeldWeapon)>()
            .without::<&Timer<ShootCooldown>>();

        let mut bullets = Vec::new();
        let mut cooldowns = Vec::new();

        for (entity, (player, input, look_dir, position, weapon)) in query {
            // Check if shooting
            if !input.shoot {
                continue;
            }

            // Spawn bullet
            let range = weapon.0.range();
            let duration = Duration::from_secs_f32(1.0);
            bullets.push(BulletSpawn {
                player: *player,
                pos: *position,
                dir: *look_dir,
                range,
                duration,
            });

            // Set cooldown
            let cooldown = weapon.0.recharge();
            cooldowns.push((entity, Timer::new(cooldown, ShootCooldown)));
        }

        for bullet in bullets {
            spawn_bullet(bullet.player, ecs, bullet.pos, bullet.dir, bullet.range, bullet.duration);
        }

        for (entity, cooldown) in cooldowns {
            ecs.world.insert_one(entity, cooldown).unwrap();
        }
    }

    pub fn shoot_cooldown_system(ecs: &mut ServerEcs, _dt: f32) {
        Timer::<ShootCooldown>::system(&mut ecs.world);
    }

    pub fn bullet_despawn_system(ecs: &mut ServerEcs, _dt: f32) {
        Timer::<BulletDespawn>::system_with(&mut ecs.world, |world, entity, _| {
            ecs.observer.observe(world).despawn(entity).unwrap();
        });
    }
}
