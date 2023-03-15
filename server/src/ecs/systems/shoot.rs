use glam::Vec2;
use crate::ecs::spawn::bullet::spawn_bullet;
use crate::ecs::systems::ServerSystems;
use crate::ecs::ServerEcs;
use common::ecs::components::{HeldWeapon, InputState, LookDirection, Position, Bullet, Health};
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
            .query_mut::<(&InputState, &LookDirection, &Position, &mut HeldWeapon)>()
            .without::<&Timer<ShootCooldown>>();

        let mut bullets = Vec::new();
        let mut cooldowns = Vec::new();

        for (entity, (input, look_dir, position, weapon)) in query {
            let mut weapon = ecs.observer.observe_component(entity, weapon);

            // Check if shooting
            if !input.shoot || weapon.ammo == 0 {
                continue;
            }

            // Spawn bullet
            bullets.push(BulletSpawn {
                pos: Position(Vec2::new(position.0.x + (look_dir.0.x * 0.4), position.0.y + (look_dir.0.y * 0.4))),
                dir: *look_dir,
                gun: weapon.gun,
            });

            // Subtract ammo
            weapon.ammo -= 1;

            // Set cooldown
            let cooldown = weapon.gun.recharge();
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

    pub fn shoot_up_system(ecs: &mut ServerEcs, _dt: f32) {
        let player_query = ecs
            .world
            .query_mut::<(&Position, &Health)>()
            .into_iter()
            .map(|(e, (&p, &w))| (e, (p, w)))
            .collect::<Vec<_>>();

        let bullet_query = ecs
            .world
            .query_mut::<(&Position, &Bullet)>()
            .into_iter()
            .map(|(e, (&p, &c))| (e, p, c))
            .collect::<Vec<_>>();

        for (e, (p, h)) in player_query {
            for c in &bullet_query {
                if (p.0.x - c.1.0.x).abs() < 0.3 && (p.0.y - c.1.0.y).abs() < 0.3 {
                    {
                        ecs.observed_world().insert(e, (
                            Health(h.0 - 1),
                        )).unwrap();
                    }
                }
            }
        }
    }
}
