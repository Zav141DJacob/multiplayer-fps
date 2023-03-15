use crate::ecs::spawn::bullet::spawn_bullet;
use crate::ecs::systems::ServerSystems;
use crate::ecs::ServerEcs;
use common::ecs::components::{ HeldWeapon, InputState, LookDirection, Position };
use common::ecs::timer::Timer;
use common::gun::Gun;
use crate::ecs::components::{BulletDespawn, ShootCooldown};

struct BulletSpawn {
    pos: Position,
    dir: LookDirection,
    gun: Gun,
}

impl ServerSystems {
    /// Applies input state to Velocity and LookDirection
    pub fn shoot_system(ecs: &mut ServerEcs, _dt: f32) {
        let query = ecs
            .world
            .query_mut::<(&InputState, &LookDirection, &Position, &HeldWeapon)>()
            .without::<&Timer<ShootCooldown>>();

        let mut bullets = Vec::new();
        let mut cooldowns = Vec::new();

        for (entity, (input, look_dir, position, weapon)) in query {
            // Check if shooting
            if !input.shoot {
                continue;
            }

            // Spawn bullet
            bullets.push(BulletSpawn {
                pos: *position,
                dir: *look_dir,
                gun: weapon.0,
            });

            // Set cooldown
            let cooldown = weapon.0.recharge();
            cooldowns.push((entity, Timer::new(cooldown, ShootCooldown)));
        }

        for bullet in bullets {
            spawn_bullet(ecs, bullet.pos, bullet.dir, bullet.gun);
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
