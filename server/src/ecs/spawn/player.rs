use crate::ecs::components::{ShotBy, Speed};
use common::ecs::components::{InputState, Velocity, Kills, Deaths};
use common::gun::Gun;
use common::{
    defaults::DEFAULT_PLAYER_HP,
    ecs::components::{Health, LookDirection, Player, Position},
    map::Map,
};
use glam::Vec2;
use hecs::Entity;

use crate::ecs::ServerEcs;

const DEFAULT_SPEED: f32 = 2.5;

pub fn spawn_player_at(pos: Position, ecs: &mut ServerEcs, username: &str) -> Entity {
    let entity = ecs.world.reserve_entity();
    // Insert observed components
    ecs.observed_world()
        .insert(
            entity,
            (
                Player {
                    id: entity.to_bits().into(),
                    name: username.to_string(),
                },
                Position(pos.0),
                Health(DEFAULT_PLAYER_HP),
                Velocity(Vec2::ZERO),
                LookDirection(Vec2::from_angle(0.0)),
                Gun::Pistol.to_held_weapon(),
                Kills(0),
                Deaths(0),
            ),
        )
        .unwrap();

    // Insert server-side components
    ecs.world
        .insert(entity, (
            ShotBy {
                id: None,
            },
            InputState::default(),
            Speed(DEFAULT_SPEED),
        ))
        .unwrap();

    entity
}

pub fn spawn_player(ecs: &mut ServerEcs, username: &str) -> (Position, Entity) {
    let pos = ecs
        .resources
        .get::<Map>()
        .unwrap()
        .random_empty_spot()
        .expect("Can't find a random spot");

    (pos, spawn_player_at(pos, ecs, username))
}
