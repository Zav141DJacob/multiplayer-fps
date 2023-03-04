use std::time::{Duration, Instant};
use common::ecs::components::{InputState, LookDirection, Position};
use crate::ecs::ServerEcs;
use crate::ecs::systems::ServerSystems;

impl ServerSystems {
    /// Applies input state to Velocity and LookDirection
    pub fn shooting_system(ecs: &mut ServerEcs, _dt: f32) {
        ecs.world.query_mut::<(&InputState, &LookDirection, &Position, &HeldWeapon)>().without::<&ShootCooldown>()
            .into_iter()
            .for_each(|(entity, (input, vel, look_dir, speed))| {
                // Check if firing

                // Spawn bullet

                // Set cooldown
                let cooldown_end = Instant::now() + Duration::from_secs_f32(1.0);
                ecs.world.insert_one(entity, ShootCooldown { end: cooldown_end });

                todo!()
            })
    }

    pub fn shoot_cooldown_system(ecs: &mut ServerEcs, _dt: f32) {
        let now = Instant::now();
        let mut to_remove = vec![];

        ecs.world.query_mut::<(&ShootCooldown)>()
            .into_iter()
            .for_each(|(entity, (cooldown))| {
                if cooldown.end < now {
                    to_remove.push(entity);
                }
            });

        for entity in to_remove {
            ecs.world.remove_one::<&ShootCooldown>(entity)
        }
    }
}