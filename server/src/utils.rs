use common::{ecs::components::{Position, Player}, map::Map};
use glam::Vec2;
use hecs::Entity;

use crate::ecs::ServerEcs;

pub fn spawn_player_at(pos: Position, ecs: &mut ServerEcs, player_id: u64) -> Entity {
    ecs.observed_world().spawn((
        Player {
            id: player_id
        },
        Position(Vec2 {
            x: pos.0.x,
            y: pos.0.y,
        }),
    ))
}

pub fn spawn_player(map: &Map, world: &mut ServerEcs, player: u64) -> (Position, Entity) {
    let pos = map.random_empty_spot();

    (pos, spawn_player_at(pos, world, player))
}
