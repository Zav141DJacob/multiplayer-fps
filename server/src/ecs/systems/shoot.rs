use crate::ecs::spawn::bullet::spawn_bullet;
use crate::ecs::systems::ServerSystems;
use crate::ecs::ServerEcs;
use common::ecs::components::{ Bullet, HeldWeapon, InputState, LookDirection, Position, ShootCooldown };
use hecs::Entity;
use std::time::{Duration, SystemTime};

struct BulletSpawn {
    pos: Position,
    dir: LookDirection,
    range: f32,
    despawn: SystemTime,
}

impl ServerSystems {
    /// Applies input state to Velocity and LookDirection
    pub fn shoot_system(ecs: &mut ServerEcs, _dt: f32) {
        let query = ecs
            .world
            .query_mut::<(&InputState, &LookDirection, &Position, &HeldWeapon)>()
            .without::<&ShootCooldown>();

        let mut bullets: Vec<BulletSpawn> = Vec::new();
        let mut cooldowns: Vec<(Entity, ShootCooldown)> = Vec::new();

        for (entity, (input, look_dir, position, weapon)) in query {
            // Check if shooting
            if !input.shoot {
                continue;
            }

            // Spawn bullet
            let range = weapon.0.range();
            let despawn = SystemTime::now() + Duration::from_secs_f32(1.0);
            bullets.push(BulletSpawn {
                pos: position.clone(),
                dir: look_dir.clone(),
                range: range,
                despawn,
            });

            // Set cooldown
            let cooldown = weapon.0.recharge();
            let cooldown_end = SystemTime::now() + cooldown;
            cooldowns.push((entity, ShootCooldown(cooldown_end)));
        }

        for bullet in bullets {
            spawn_bullet(ecs, bullet.pos, bullet.dir, bullet.range, bullet.despawn);
        }

        for cooldown in cooldowns {
            ecs.world.insert_one(cooldown.0, cooldown.1).unwrap();
        }
    }

    pub fn shoot_cooldown_system(ecs: &mut ServerEcs, _dt: f32) {
        let mut to_remove = vec![];
        let now = SystemTime::now();

        ecs.world
            .query_mut::<&ShootCooldown>()
            .into_iter()
            .filter(|(_, cooldown)| cooldown.0 < now)
            .for_each(|(entity, _)| {
                to_remove.push(entity);
            });

        for entity in to_remove {
            ecs.world.remove_one::<&ShootCooldown>(entity).unwrap();
        }
    }

    pub fn bullet_despawn_system(ecs: &mut ServerEcs, _dt: f32) {
        let mut to_remove = vec![];
        let now = SystemTime::now();

        ecs.world
            .query_mut::<&Bullet>()
            .into_iter()
            .filter(|(_, bullet)| bullet.despawn_time < now)
            .for_each(|(entity, _)| {
                to_remove.push(entity);
            });

        for entity in to_remove {
            ecs.world.despawn(entity).unwrap();
        }
    }
}
