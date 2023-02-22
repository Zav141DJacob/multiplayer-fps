use common::{ecs::components::{Position, Player, LookDirection}, map::Map};
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
        LookDirection(0.0)
    ))
}

pub fn spawn_player(map: &Map, ecs: &mut ServerEcs, player: u64) -> (Position, Entity) {
    let pos = map.random_empty_spot();

    (pos, spawn_player_at(pos, ecs, player))
}
