use itertools::Itertools;
use common::ecs::components::{Player, Position, Velocity};
use crate::game::ecs::ClientEcs;
use crate::game::ecs::component::{ClientInitialized, ClientSide, RenderSprite};
use crate::game::ecs::systems::ClientSystems;
use crate::game::texture::animated_texture::AnimatedTextureState;
use crate::game::texture::{ANIMATED_PLAYER, ATLAS_MONSTER};

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
}