use glam::Vec2;

use common::ecs::components::{InputState, LookDirection, Velocity};

use crate::ecs::components::Speed;
use crate::ecs::ServerEcs;
use crate::ecs::systems::ServerSystems;

impl ServerSystems {
    /// Applies input state to Velocity and LookDirection
    pub fn input_system(ecs: &mut ServerEcs, _dt: f32) {
        let query = ecs.world.query_mut::<(&InputState, &mut Velocity, &mut LookDirection, &Speed)>();

        for (entity, (input, vel, look_dir, speed)) in query {
            // Apply look_direction
            // Using a block so look_dir gets dropped before observing vel
            {
                let mut look_dir = ecs.observer.observe_component(entity, look_dir);
                look_dir.0 = Vec2::from_angle(input.look_angle);
            }

            // Apply velocity
            let mut move_dir = Vec2::ZERO;

            let forward = look_dir.0;
            let right = forward.perp();

            if input.forward {
                move_dir += forward;
            }

            if input.backward {
                move_dir -= forward;
            }

            if input.right {
                move_dir += right;
            }

            if input.left {
                move_dir -= right;
            }

            // this makes backend and frontend crash, so I commented it out for now -Jacob
            move_dir = move_dir.normalize_or_zero();

            let mut vel = ecs.observer.observe_component(entity, vel);
            vel.0 = move_dir * speed.0;
        }
    }
}
