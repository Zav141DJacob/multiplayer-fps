use glam::Vec2;
use itertools::Itertools;
use common::ecs::components::{Bullet, Player};
use crate::game::ecs::ClientEcs;
use crate::game::ecs::component::{ClientInitialized, Height, RenderSprite, Scale};
use crate::game::ecs::systems::ClientSystems;
use crate::game::texture::{ANIMATED_PLAYER, TEX_BULLET};

impl ClientSystems {
    /// Move all entities with a position and velocity
    pub fn init_player(ecs: &mut ClientEcs, _dt: f32) {
        let entities = ecs.world.query_mut::<()>().with::<&Player>().without::<&ClientInitialized>()
            .into_iter()
            .map(|(ent, _)| ent)
            .collect_vec();

        for entity in entities {
            ecs.world.insert(entity, (
                ANIMATED_PLAYER.get_state("standing"),
                ClientInitialized,
            )).unwrap();
        }
    }

    pub fn init_bullet(ecs: &mut ClientEcs, _dt: f32) {
        let entities = ecs.world.query_mut::<()>().with::<&Bullet>().without::<&ClientInitialized>()
            .into_iter()
            .map(|(ent, _)| ent)
            .collect_vec();

        const BULLET_SCALE: f32 = 4.0 / 64.0;
        for entity in entities {
            ecs.world.insert(entity, (
                RenderSprite { tex: &TEX_BULLET },
                Scale(Vec2::splat(BULLET_SCALE)),
                Height(0.5 - BULLET_SCALE / 2.0),
                ClientInitialized,
            )).unwrap();
        }
    }
}