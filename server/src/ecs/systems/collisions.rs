use std::collections::HashMap;
// use std::iter::Map;

use crate::ecs::ServerEcs;
use crate::{ecs::systems::ServerSystems};
use common::defaults::PLAYER_SIZE;
use common::ecs::components::{Position, Player, Bullet};
use common::map::{Map, MapCell};
use glam::Vec2;
use hecs::{QueryMut, Entity};
use std::any::type_name;
pub enum Direction {
    Up,
    Right,
    Left,
    Down,
}

impl ServerSystems {
    pub fn collision_system(ecs: &mut ServerEcs, _dt: f32) {

        // let bullet_query = ecs.world.query::<(&Bullet, &mut Position)>();

        Self::prepare_wall_collisions::<&Player>(ecs);
        Self::prepare_wall_collisions::<&Bullet>(ecs);
        // Self::bullet_player_collisions(ecs);
    }

    // fn bullet_player_collisions(ecs: &mut ServerEcs) {
    //     let query = ecs.world.query_mut::<(&, &mut Position)>();

    // }

    fn prepare_wall_collisions<T: hecs::Query + hecs::Component>(ecs: &mut ServerEcs) {
        let generic_type = type_name::<T>().split("::").last().unwrap();
        let query = ecs.world.query_mut::<(T, &mut Position)>();
        let map = ecs.resources.get::<Map>().unwrap().clone();
        let mut player_positions: Vec<Vec2> = Vec::new();
        let mut to_remove: Vec<Entity> = Vec::new();
        for (entity, (_, pos)) in query {
            let mut pos = ecs.observer.observe_component(entity, pos);
            let pos = &mut pos.0;
            match generic_type {
                "Player" => {
                    let to_pos = Self::wall_collision(map.clone(), pos, PLAYER_SIZE);

                    player_positions.push(to_pos);
                    *pos = to_pos;
                },
                "Bullet" => {
                    let to_pos = Self::wall_collision(map.clone(), pos, 0.0);

                    if *pos != to_pos {
                        to_remove.push(entity);
                    }
                },
                _ => panic!()
            }
        }
        for e in to_remove {
            ecs.observed_world().despawn(e).unwrap();
        }
        to_remove = Vec::new();
        if generic_type == "Player" {
            let bullet_query = ecs.world.query_mut::<(&Bullet, &Position)>();

            for (entity, (_, bullet_pos)) in bullet_query {
                for player_pos in &player_positions {
                    // if player_pos.distance(bullet_pos.0) > 0.0 {
                    //     // todo
                    //     //  add take damage function here
                    //     to_remove.push(entity);
                    // }
                }
            }
        }
        for e in to_remove {
            ecs.observed_world().despawn(e).unwrap();
        }

    }

    fn wall_collision(map: Map, pos: &mut Vec2, size: f32) -> Vec2{

            let mut to_pos = pos.clone();

            let x_floored_int = pos.x.floor() as i32;
            let y_floored_int = pos.y.floor() as i32;

            let x_floored_f = pos.x.floor();
            let y_floored_f = pos.y.floor();

            // let cell_position = Vec2::new(x_floored, y_floored);

            // dbg!(y_floored_int);
            let cells: Vec<(Vec2, MapCell)> = vec![
                (Vec2::new(x_floored_f, y_floored_f + 1.0), map.cell(x_floored_int as usize, (y_floored_int + 1) as usize)),
                (Vec2::new(x_floored_f + 1.0, y_floored_f + 1.0), map.cell((x_floored_int + 1) as usize, (y_floored_int + 1) as usize)),
                (Vec2::new(x_floored_f + 1.0, y_floored_f), map.cell((x_floored_int + 1) as usize, y_floored_int as usize)),
                (Vec2::new(x_floored_f + 1.0, y_floored_f - 1.0), map.cell((x_floored_int + 1) as usize, (y_floored_int - 1) as usize)),
                (Vec2::new(x_floored_f, y_floored_f - 1.0), map.cell(x_floored_int as usize, (y_floored_int - 1) as usize)),
                (Vec2::new(x_floored_f - 1.0, y_floored_f - 1.0), map.cell((x_floored_int - 1) as usize, (y_floored_int - 1) as usize)),
                (Vec2::new(x_floored_f - 1.0, y_floored_f), map.cell((x_floored_int - 1) as usize, y_floored_int as usize)),
                (Vec2::new(x_floored_f - 1.0, y_floored_f + 1.0), map.cell((x_floored_int - 1) as usize, (y_floored_int + 1) as usize)),
            ];
            
            for (i, (cell_pos, cell)) in cells.iter().enumerate() {
                if let MapCell::Empty = cell {
                } else {


                    let side = Self::side_from_usize(i);
                    if let Some(side) = side {
                        let side_vec = Self::side_vec_from_side(cell_pos, &side);
                        if let Some(side_vec) = side_vec {
                            if Self::in_circle(&side_vec, &pos) {
                                match side {
                                    Direction::Up => {
                                        to_pos.y = y_floored_f + (1.0 - size / 2.0);
                                        to_pos.x = pos.x;
                                    },
                                    Direction::Right => {
                                        to_pos.x = x_floored_f + (size / 2.0);
                                        to_pos.y = pos.y;
                                    },
                                    Direction::Down => {
                                        to_pos.y = y_floored_f + (size / 2.0);
                                        to_pos.x = pos.x;
                                    },
                                    Direction::Left => {
                                        to_pos.x = x_floored_f + (1.0 - size / 2.0); 
                                        to_pos.y = pos.y;
                                    }
                                }
                            }
                        }
                    }
                    
                }
            }
            to_pos
        // }
    }

    //  Checks if a line is inside a circle
    // takes in the line and a position as a Vec2
    // https://math.stackexchange.com/questions/275529/check-if-line-intersects-with-circles-perimeter
    pub fn in_circle(line: &[Vec2; 2], player_pos: &Vec2) -> bool{
        let radius = PLAYER_SIZE / 2.0;

        let a = line[0].x - line[1].x;
        let b = line[0].y - line[1].y;
        let x = (a*a + b*b).sqrt();
        return ((player_pos.x - line[0].x) * (line[1].y - line[0].y) - (player_pos.y -  line[0].y) * (line[1].x - line[0].x)).abs() / x <= radius;
    }

    pub fn side_vec_from_side(position: &Vec2, side: &Direction) -> Option<[Vec2; 2]> {
        
        match side {
            Direction::Up => Some([
                Vec2::new(position.x, position.y),
                Vec2::new(position.x + 1.0, position.y)
            ]),
            Direction::Right => Some([
                Vec2::new(position.x + 1.0, position.y),
                Vec2::new(position.x + 1.0, position.y - 1.0)
            ]),
            Direction::Down => Some([
                Vec2::new(position.x, position.y + 1.0),
                Vec2::new(position.x + 1.0, position.y + 1.0)
            ]),
            Direction::Left => Some([
                Vec2::new(position.x, position.y),
                Vec2::new(position.x, position.y - 1.0)
            ]),
            _ => None
        }
    }
    pub fn side_from_usize(i: usize) -> Option<Direction>{
        // 1 - down
        // 3 - left
        // 5 - up
        // 7 - right
        match i {
            0 => Some(Direction::Up),
            2 => Some(Direction::Left),
            4 => Some(Direction::Down),
            6 => Some(Direction::Right),
            _ => None
        }
    }
}
