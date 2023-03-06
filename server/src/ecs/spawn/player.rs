use common::{ecs::components::{Position, Player, LookDirection, HeldWeapon}, map::Map};
use common::gun::Gun;
use glam::Vec2;
use hecs::Entity;
use common::ecs::components::{InputState, Velocity};
use crate::ecs::components::Speed;

use crate::ecs::ServerEcs;

const DEFAULT_SPEED: f32 = 2.5;

pub fn spawn_player_at(pos: Position, ecs: &mut ServerEcs) -> Entity {
    let entity = ecs.world.reserve_entity();

    // Insert observed components
    ecs.observed_world().insert(entity, (
        Player {
            id: entity.to_bits().into()
        },
        Position(pos.0),
        Velocity(Vec2::ZERO),
        LookDirection(Vec2::from_angle(0.0)),
        HeldWeapon(Gun::Pistol),
    )).unwrap();

    // Insert server-side components
    ecs.world.insert(entity, (
        InputState::default(),
        Speed(DEFAULT_SPEED),
    )).unwrap();

    entity
}

pub fn spawn_player(ecs: &mut ServerEcs) -> (Position, Entity) {
    let pos = ecs.resources.get::<Map>().unwrap().random_empty_spot();

    (pos, spawn_player_at(pos, ecs))
}
