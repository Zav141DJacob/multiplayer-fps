use crate::ecs::ServerEcs;
use crate::{ecs::systems::ServerSystems};
use common::ecs::components::{Position, Player};
use common::map::{Map, MapCell};
use glam::Vec2;
use hecs::QueryMut;

impl ServerSystems {
    pub fn collision_system(ecs: &mut ServerEcs, _dt: f32) {
        Self::wall_collisions(ecs);
    }

    fn wall_collisions(ecs: &mut ServerEcs) {
        let player_query = ecs.world.query_mut::<(&Player, &mut Position)>();
        let map = (ecs.resources.get::<Map>().unwrap()).clone();
        for (entity, (player, pos)) in player_query {
            let x_floored = pos.0.x.floor() as usize;
            let y_floored = pos.0.y.floor() as usize;

            // let cell_position = Vec2::new(x_floored, y_floored);

            let up = map.cell(x_floored, y_floored + 1);
            let right = map.cell(x_floored + 1, y_floored);
            let down = map.cell(x_floored, y_floored - 1);
            let left = map.cell(x_floored - 1, y_floored);

            if let MapCell::Empty = up {
            } else {
                let rect = Vec2::new(x_floored as f32, (y_floored + 1) as f32);
            }
            
        }
    }
}
