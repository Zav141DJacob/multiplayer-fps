use glam::Vec2;
use common::ecs::components::{InputState, LookDirection, Velocity};
use crate::ecs::components::Speed;
use crate::ecs::ServerEcs;
use crate::ecs::systems::ServerSystems;

impl ServerSystems {
    /// Applies input state to Velocity and LookDirection
    pub fn input_system(ecs: &mut ServerEcs, _dt: f32) {
        ecs.world.query_mut::<(&InputState, &mut Velocity, &mut LookDirection, &Speed)>()
            .into_iter()
            .for_each(|(entity, (input, vel, look_dir, speed))| {
                // Observe the components we mutate
                let mut vel = ecs.observer.observe_component(entity, vel);
                let mut look_dir = ecs.observer.observe_component(entity, look_dir);

                // Unwrap component inner types
                let look_dir = &mut look_dir.0;
                let vel = &mut vel.0;
                let speed = speed.0;

                // Apply look_direction
                *look_dir = Vec2::from_angle(input.look_angle);

                // Apply velocity
                let mut move_dir = Vec2::ZERO;
                
                let forward = *look_dir;
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
                
                move_dir = move_dir.normalize();
                
                *vel = move_dir * speed;
            })
    }
}