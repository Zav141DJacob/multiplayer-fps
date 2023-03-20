use glam::Vec2;
use itertools::Itertools;
use common::ecs::components::{Bullet, DeadPlayer, Player, WeaponCrate};
use crate::game::ecs::ClientEcs;
use crate::game::ecs::component::{ClientInitialized, Height, RenderSprite, Scale};
use crate::game::ecs::systems::ClientSystems;
use crate::game::texture::{ANIMATED_DEATH, ANIMATED_PLAYER, TEX_BULLET, WEAPON_CRATE};

impl ClientSystems {
    pub fn client_init(ecs: &mut ClientEcs, _dt: f32) {
        Self::init_player(ecs);
        Self::init_bullet(ecs);
        Self::init_crate(ecs);
        Self::init_death(ecs);
    }

    /// Move all entities with a position and velocity
    fn init_player(ecs: &mut ClientEcs) {
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

    fn init_bullet(ecs: &mut ClientEcs) {
        let entities = ecs.world.query_mut::<()>().with::<&Bullet>().without::<&ClientInitialized>()
            .into_iter()
            .map(|(ent, _)| ent)
            .collect_vec();

        const BULLET_SCALE: f32 = 4.0 / 64.0;
        for entity in entities {
            ecs.world.insert(entity, (
                RenderSprite { tex: &TEX_BULLET },
                Scale(Vec2::splat(BULLET_SCALE)),
                Height(0.45 - BULLET_SCALE / 2.0),
                ClientInitialized,
            )).unwrap();
        }
    }

    fn init_crate(ecs: &mut ClientEcs) {
        let entities = ecs.world.query_mut::<()>().with::<&WeaponCrate>().without::<&ClientInitialized>()
            .into_iter()
            .map(|(ent, _)| ent)
            .collect_vec();

        for entity in entities {
            ecs.world.insert(entity, (
                RenderSprite { tex: &WEAPON_CRATE },
                Scale(Vec2::splat(0.5)),
                ClientInitialized,
            )).unwrap();
        }
    }

    fn init_death(ecs: &mut ClientEcs) {
        let entities = ecs.world.query_mut::<()>().with::<&DeadPlayer>().without::<&ClientInitialized>()
            .into_iter()
            .map(|(ent, _)| ent)
            .collect_vec();

        for entity in entities {
            ecs.world.insert(entity, (
                ANIMATED_DEATH.get_state("dying"),
                ClientInitialized,
            )).unwrap();
        }
    }
}