use common::{
    ecs::components::{LookDirection, Player},
    Direction,
};
use glam::Vec2;

use crate::ecs::ServerEcs;

const CAMERA_SENSITIVITY: f32 = 0.08; // rad

pub fn execute(ecs: &mut ServerEcs, direction: Direction, requester_id: u64) {
    // if server.is_registered(requester_id) {

    let (entity, (_, look_direction)) = ecs
        .world
        .query_mut::<(&Player, &mut LookDirection)>()
        .into_iter()
        .find(|(_, (&player, _))| player.id == requester_id)
        .unwrap();

    let mut dir = *look_direction;

    // if app.keyboard.is_down(KeyCode::Left) {
    //     dir.0 = dir.0.rotate(Vec2::from_angle(-CAMERA_SENSITIVITY));
    // }

    // if app.keyboard.is_down(KeyCode::Right) {
    //     dir.0 = dir.0.rotate(Vec2::from_angle(CAMERA_SENSITIVITY));
    // }
    match direction {
        common::Direction::Left => {
            dir.0 = dir.0.rotate(Vec2::from_angle(-CAMERA_SENSITIVITY));
        }
        common::Direction::Right => {
            dir.0 = dir.0.rotate(Vec2::from_angle(CAMERA_SENSITIVITY));
        }
        _ => panic!(),
    };

    // *server.observer.observe_component(entity, position) = pos;
    *ecs.observer.observe_component(entity, look_direction) = dir;

    // }
}
