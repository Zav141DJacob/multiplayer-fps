use common::{
    defaults::PLAYER_SPEED,
    ecs::components::{LookDirection, Player, Position},
    map::Map,
    Direction,
};

use crate::{ecs::ServerEcs, server::Logger};

pub fn execute(ecs: &mut ServerEcs, direction: Direction, requester_id: u64) {
    let logger = ecs.resources.get::<Logger>().unwrap();
    logger.log(format!("move {direction:?}"));

    let (entity, (_, look_direction, position)) = ecs
        .world
        .query_mut::<(&Player, &mut LookDirection, &mut Position)>()
        .into_iter()
        .find(|(_, (&player, _, _))| player.id == requester_id)
        .unwrap();

    let mut pos = *position;
    let dir = *look_direction;

    let map = ecs.resources.get::<Map>().unwrap();
    let w = map.get_width() as f32;
    let h = map.get_height() as f32;

    match direction {
        common::Direction::Forward => {
            if pos.0.x + dir.0.x * PLAYER_SPEED < 0.0 {
                pos.0.x = 0.0;
            } else if pos.0.x + dir.0.x * PLAYER_SPEED > w {
                pos.0.x = w;
            } else {
                pos.0.x += dir.0.x * PLAYER_SPEED;
            }

            if pos.0.y - dir.0.y * PLAYER_SPEED < 0.0 {
                pos.0.y = 0.0;
            } else if pos.0.y - dir.0.y * PLAYER_SPEED > h {
                pos.0.y = h;
            } else {
                pos.0.y -= dir.0.y * PLAYER_SPEED;
            }
        }
        common::Direction::Backward => {
            if pos.0.x - dir.0.x * PLAYER_SPEED < 0.0 {
                pos.0.x = 0.0;
            } else if pos.0.x - dir.0.x * PLAYER_SPEED > w {
                pos.0.x = w;
            } else {
                pos.0.x -= dir.0.x * PLAYER_SPEED;
            }

            if pos.0.y + dir.0.y * PLAYER_SPEED < 0.0 {
                pos.0.y = 0.0;
            } else if pos.0.y + dir.0.y * PLAYER_SPEED > h {
                pos.0.y = h;
            } else {
                pos.0.y += dir.0.y * PLAYER_SPEED;
            }
        }
        common::Direction::Left => {
            if pos.0.x - dir.0.y * PLAYER_SPEED < 0.0 {
                pos.0.x = 0.0;
            } else if pos.0.x - dir.0.y * PLAYER_SPEED > w {
                pos.0.x = w;
            } else {
                pos.0.x -= dir.0.y * PLAYER_SPEED;
            }

            if pos.0.y - dir.0.x * PLAYER_SPEED < 0.0 {
                pos.0.y = 0.0;
            } else if pos.0.y - dir.0.x * PLAYER_SPEED > h {
                pos.0.y = h;
            } else {
                pos.0.y -= dir.0.x * PLAYER_SPEED;
            }
        }
        common::Direction::Right => {
            if pos.0.x + dir.0.y * PLAYER_SPEED < 0.0 {
                pos.0.x = 0.0;
            } else if pos.0.x + dir.0.y * PLAYER_SPEED > w {
                pos.0.x = w;
            } else {
                pos.0.x += dir.0.y * PLAYER_SPEED;
            }

            if pos.0.y + dir.0.x * PLAYER_SPEED < 0.0 {
                pos.0.y = 0.0;
            } else if pos.0.y + dir.0.x * PLAYER_SPEED > h {
                pos.0.y = h;
            } else {
                pos.0.y += dir.0.x * PLAYER_SPEED;
            }
        }
    };

    logger.log(format!("Player position: {pos:?}"));

    *ecs.observer.observe_component(entity, position) = pos;
}
