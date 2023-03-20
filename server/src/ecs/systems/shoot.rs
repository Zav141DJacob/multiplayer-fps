use glam::Vec2;
use crate::ecs::spawn::bullet::spawn_bullet;
use crate::ecs::systems::ServerSystems;
use crate::ecs::ServerEcs;
use common::ecs::components::{HeldWeapon, InputState, LookDirection, Position, Player};
use common::ecs::timer::Timer;
use common::gun::Gun;
use crate::ecs::components::{BulletDespawn, ShootCooldown};

struct BulletSpawn {
    player: Player,
    pos: Position,
    dir: LookDirection,
    gun: Gun,
}

impl ServerSystems {
    /// Applies input state to Velocity and LookDirection
    pub fn shoot_system(ecs: &mut ServerEcs, _dt: f32) {
        let query = ecs
            .world
            .query_mut::<(&Player, &InputState, &LookDirection, &Position, &mut HeldWeapon)>()
            .without::<&Timer<ShootCooldown>>();

        let mut bullets = Vec::new();
        let mut cooldowns = Vec::new();

        for (entity, (player, input, look_dir, position, weapon)) in query {
            let mut weapon = ecs.observer.observe_component(entity, weapon);

            // Check if shooting
            if !input.shoot || weapon.gun.max_ammo() != 0 && weapon.ammo == 0 {
                continue;
            }

            // Spawn bullet
            bullets.push(BulletSpawn {
                player: player.clone(),
                pos: Position(Vec2::new(position.0.x + (look_dir.0.x * 0.4), position.0.y + (look_dir.0.y * 0.4))),
                dir: *look_dir,
                gun: weapon.gun,
            });

            // Subtract ammo
            weapon.ammo = weapon.ammo.saturating_sub(1);

            // Set cooldown
            let cooldown = weapon.gun.recharge();
            cooldowns.push((entity, Timer::new(cooldown, ShootCooldown)));
        }

        for bullet in bullets {
            spawn_bullet(ecs, bullet.player, bullet.pos, bullet.dir, bullet.gun);
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
