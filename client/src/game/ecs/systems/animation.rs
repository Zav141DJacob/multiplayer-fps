use common::ecs::components::{Position, Velocity, LookDirection, Player};
use glam::Vec2;
use crate::game::ecs::{ClientEcs, MyEntity};
use crate::game::ecs::component::{RenderSprite};
use crate::game::ecs::systems::ClientSystems;
use crate::game::texture::TEX_TEST1;
use crate::game::texture::animated_texture::AnimatedTextureState;

impl ClientSystems {
    /// Give move animation to all sprites with the move animation sprite sheet
    pub fn apply_animations(ecs: &mut ClientEcs, dt: f32) {
        let my_entity = ecs.resources.get::<MyEntity>().unwrap().0;
        let my_pos = ecs.world.query_one_mut::<&Position>(my_entity).unwrap().0;

        #[allow(clippy::needless_collect)]
        let render_spriteless: Vec<_>  = ecs.world.query_mut::<&mut AnimatedTextureState>().without::<&RenderSprite>()
            .into_iter()
            .map(|(entity, _)| {
                entity
            }).collect();

        render_spriteless.into_iter().for_each(|entity| {
            ecs.world.insert_one(entity, RenderSprite {
                tex: &TEX_TEST1
            }).unwrap();
        });





        ecs.world.query_mut::<(&mut AnimatedTextureState, &mut RenderSprite, &Position, &LookDirection )>()
            .into_iter()
            .for_each(|(_entity, (animated_texture_state, sprite, pos, look_dir))| {
                if my_pos == pos.0 {
                    return
                }

                let look_angle = -look_dir.0.angle_between(my_pos - pos.0);

                sprite.tex = animated_texture_state.get_sprite(look_angle, dt);
            });

        ecs.world.query_mut::<(&mut AnimatedTextureState, &mut RenderSprite)>().without::<&LookDirection>()
            .into_iter()
            .for_each(|(_entity, (animated_texture_state, sprite))| {
                sprite.tex = animated_texture_state.get_sprite(0.0, dt);
            });
    }

    pub fn animate_running(ecs: &mut ClientEcs, _dt: f32) {
        ecs.world.query_mut::<(&mut AnimatedTextureState, &Velocity)>().with::<&Player>()
            .into_iter()
            .for_each(|(_, (anim, vel))| {
                if vel.0 == Vec2::ZERO {
                    anim.set_state("standing", 1.0);
                } else {
                    anim.set_state("running", 1.0);
                }
            });
    }
}