use crate::ecs::systems::ServerSystems;
use crate::ecs::ServerEcs;
use common::defaults::PLAYER_SIZE;
use common::ecs::components::{Position, Player, Bullet, WithId, Health, Damage};
use common::map::{Map, MapCell, Wall};
use glam::Vec2;
use hecs::Entity;
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

        Self::prepare_wall_collisions::<Player>(ecs);
        Self::prepare_wall_collisions::<Bullet>(ecs);
        // Self::bullet_player_collisions(ecs);
    }

    // fn bullet_player_collisions(ecs: &mut ServerEcs) {
    //     let query = ecs.world.query_mut::<(&, &mut Position)>();

    // }

    fn prepare_wall_collisions<T: hecs::Component + Clone + WithId>(ecs: &mut ServerEcs) {
        let generic_type = type_name::<T>().split("::").last().unwrap();
        let map = ecs.resources.get::<Map>().unwrap().clone();
        // let mut player_positions: Vec<(T, Health, Vec2)> = Vec::new();
        let mut to_remove: Vec<Entity> = Vec::new();


        let mut bullet_positions: Vec<(Entity, Bullet, Vec2, Damage)> = Vec::new();


        if generic_type == "Player" {
            let bullet_query = ecs.world.query_mut::<(&Bullet, &Position, &Damage)>();
            for (entity, (bullet, bullet_pos, damage)) in bullet_query {
                bullet_positions.push((entity, *bullet, bullet_pos.0, *damage));
            }
        }
        let query = ecs.world.query_mut::<(&T, &mut Health, &mut Position)>();

        for (entity, (t, health, pos)) in query {
            
            match generic_type {
                "Player" => {
                    let to_pos = Self::wall_collision(map.clone(), &pos.0, PLAYER_SIZE);

                    {
                        let mut pos = ecs.observer.observe_component(entity, pos);
                        let pos = &mut pos.0;


                        *pos = to_pos;
                    }
                    {
                        
                        for (bullet_entity, bullet, bullet_pos, damage) in &bullet_positions {
                            if bullet_pos.distance(to_pos) < PLAYER_SIZE / 2.0 {
        
                                if t.id() != bullet.id() {
                                    to_remove.push(*bullet_entity);

                                    let mut health = ecs.observer.observe_component(entity, health);
                                    let health = &mut health.0;
                                    dbg!(damage);
                                    *health = health.clone() - damage.0 as u32;
                                }
                            }
                        }
                    }
                }
                "Bullet" => {
                    let to_pos = Self::wall_collision(map.clone(), &pos.0, 0.0);

                    if pos.0 != to_pos {
                        to_remove.push(entity);
                    }
                }
                _ => panic!(),
            }
        }
        for e in to_remove {
            ecs.observed_world().despawn(e);
        }
    }

    fn wall_collision(map: Map, pos: &Vec2, size: f32) -> Vec2{

        let mut to_pos = pos.clone();

        let x_floored_int = pos.x.floor() as i32;
        let y_floored_int = pos.y.floor() as i32;

        let x_floored_f = pos.x.floor();
        let y_floored_f = pos.y.floor();

        let sides: Vec<(Vec2, MapCell)> = Self::get_sides(
            &map, x_floored_f, 
            y_floored_f, 
            x_floored_int as usize, 
            y_floored_int as usize
        );
        
        let corners: Vec<(Vec2, MapCell)> = Self::get_corners(
            &map, x_floored_f, 
            y_floored_f, 
            x_floored_int as usize, 
            y_floored_int as usize
        );

        // Sides
        for (i, (cell_pos, cell)) in sides.iter().enumerate() {
            if let MapCell::Empty = cell {
            } else {
                let side_vec = Self::side_vec_from_usize(cell_pos, i);
                if let Some(side_vec) = side_vec {
                    if Self::in_circle(&side_vec, &to_pos) {
                        match i {
                            0 => {
                                to_pos.y = y_floored_f + (1.0 - size / 2.0);
                                to_pos.x = to_pos.x;
                            },
                            3 => {
                                to_pos.x = x_floored_f + (size / 2.0);
                                to_pos.y = to_pos.y;
                            },
                            2 => {
                                to_pos.y = y_floored_f + (size / 2.0);
                                to_pos.x = to_pos.x;
                            },
                            1 => {
                                to_pos.x = x_floored_f + (1.0 - size / 2.0); 
                                to_pos.y = to_pos.y;
                            },
                            _ => panic!("AAAAAAAAAAAAAAA")
                        }
                    }
                }
            }
        }

        // corners
        for (i, (cell_pos, cell)) in corners.iter().enumerate() {
            if let MapCell::Empty = cell {
            } else {
                let corner = Self::corner_from_usize(cell_pos, i);
                if let Some(corner) = corner {
                    let distance = to_pos.distance(corner);
                    if distance < size / 2.0 - 0.001 {
                        // size / 2.0
                        // to_pos = pos.rotate(Vec2::from_angle(pos.angle_between(corner)));

                        // dbg!(pos.dot(corner));

                        let dink = (to_pos.y - corner.y).abs();
                        let a: f32;
                        if i == 0 || i == 3 {
                            a = Vec2::new(corner.x, corner.y - dink).distance(to_pos);
                        } else {
                            a = Vec2::new(corner.x, corner.y + dink).distance(to_pos);
                        }
                        let b = ((size/2.0).powf(2.0) - a.powf(2.0)).sqrt();
                        let change_y = b - dink;

                        let a = change_y;
                        let r = size / 2.0;
                        let d = pos.distance(corner);

                        let mut to_acos = (d.powf(2.0) + a.powf(2.0) - r.powf(2.0)) / (2.0 * a * d);
                        if to_acos < -1.0 || to_acos > 1.0 {
                            to_acos = to_acos - to_acos % 1.0;
                        }

                        let alpha = 360.0 - 90.0 - 
                        to_acos.acos();

                        dbg!(alpha);

                        let determinant = (2.0 * d * alpha.cos()).powf(2.0) + 4.0 * (r.powf(2.0) - d.powf(2.0));
                        dbg!(determinant);
                        let x_1 = 
                        (
                            2.0 * d * alpha.cos() + determinant.sqrt()
                        ) / 2.0; 
                        let x_2 = 
                        (
                            2.0 * d * alpha.cos() - determinant.sqrt()
                        ) / 2.0; 

                        let x_max = x_1.max(x_2);
                        dbg!(x_1, x_2);
                        let change_x: f32;
                        // if i == 0 || i == 1 {
                        //     change_x = Vec2::new(corner.x + x_max, corner.y).distance(*pos);
                        // } else {
                        //     change_x = Vec2::new(corner.x - x_max, corner.y).distance(*pos);
                        // }
                        change_x = x_max;
                        dbg!(change_x, change_y);
                        // dbg!(pos.angle_between(corner).to_degrees());
                        // let angle_in_between = 90.0 * distance;
                        match i {
                            0 => {
                                to_pos.y = to_pos.y - change_y;
                                to_pos.x = to_pos.x - change_x;
                            },
                            1 => {
                                to_pos.y = to_pos.y + change_y;
                                to_pos.x = to_pos.x - change_x;
                            },
                            2 => {
                                to_pos.y = to_pos.y + change_y;
                                to_pos.x = to_pos.x + change_x;
                            },

                            // done, dont change
                            3 => {
                                to_pos.y = to_pos.y - change_y; 
                                to_pos.x = to_pos.x + change_x;
                            },
                            _ => panic!("AAAAAAAAAAAAAAA")
                        }
                        dbg!(to_pos);
                    }
                }
            }
        }
        to_pos
    }

    fn get_sides(map: &Map, x_f: f32, y_f: f32, x_i: usize, y_i: usize) -> Vec<(Vec2, MapCell)>{
        let mut return_vec: Vec<(Vec2, MapCell)> = Vec::new();
        if y_i + 1 >= map.get_height() {
            return_vec.push(
                (Vec2::new(x_f, y_f + 1.0), MapCell::Empty)
            );
        } else {
            return_vec.push(
                (Vec2::new(x_f, y_f + 1.0), map.cell(x_i, y_i + 1))
            );
        }

        if x_i + 1 >= map.get_width() {
            return_vec.push(
                (Vec2::new(x_f + 1.0, y_f), MapCell::Empty)
            );
        } else {
            return_vec.push(
                (Vec2::new(x_f + 1.0, y_f), map.cell(x_i + 1, y_i))
            );
        }

        if y_i as i32 - 1 < 0 {
            return_vec.push(
                (Vec2::new(x_f, y_f - 1.0), MapCell::Empty)
            );
        } else {
            return_vec.push(
                (Vec2::new(x_f, y_f - 1.0), map.cell(x_i, y_i - 1))
            );
        }
        
        if x_i as i32 - 1 < 0 {
            return_vec.push(
                (Vec2::new(x_f - 1.0, y_f), MapCell::Empty)
            );
        } else {
            return_vec.push(
                (Vec2::new(x_f - 1.0, y_f), map.cell(x_i - 1, y_i))
            );
        }
        return_vec
    }

    fn get_corners(map: &Map, x_f: f32, y_f: f32, x_i: usize, y_i: usize) -> Vec<(Vec2, MapCell)> {
        let mut return_vec: Vec<(Vec2, MapCell)> = Vec::new();
        if x_i + 1 >= map.get_width() || y_i + 1 >= map.get_height() {
            return_vec.push(
                (Vec2::new(x_f + 1.0, y_f + 1.0), MapCell::Wall(Wall::default()))
            );
        } else {
            return_vec.push(
                (Vec2::new(x_f + 1.0, y_f + 1.0), map.cell(x_i + 1, y_i + 1))
            );
        }

        if x_i + 1 >= map.get_width() || y_i as i32 - 1 < 0 {
            return_vec.push(
                (Vec2::new(x_f + 1.0, y_f - 1.0), MapCell::Wall(Wall::default()))
            );
        } else {
            return_vec.push(
                (Vec2::new(x_f + 1.0, y_f - 1.0), map.cell(x_i + 1, y_i - 1))
            );
        }

        if x_i as i32 - 1 < 0 || y_i as i32 - 1 < 0 {
            return_vec.push(
                (Vec2::new(x_f - 1.0, y_f - 1.0), MapCell::Wall(Wall::default()))
            );
        } else {
            return_vec.push(
                (Vec2::new(x_f - 1.0, y_f - 1.0), map.cell(x_i - 1, y_i - 1))
            );
        }
        
        if x_i as i32 - 1 < 0 || y_i + 1 >= map.get_height() {
            return_vec.push(
                (Vec2::new(x_f - 1.0, y_f + 1.0), MapCell::Wall(Wall::default()))
            );
        } else {
            return_vec.push(
                (Vec2::new(x_f - 1.0, y_f + 1.0), map.cell(x_i - 1, y_i + 1))
            );
        }
        return_vec
    }

    //  Checks if a line is inside a circle
    // takes in the line and a position as a Vec2
    // https://math.stackexchange.com/questions/275529/check-if-line-intersects-with-circles-perimeter
    pub fn in_circle(line: &[Vec2; 2], player_pos: &Vec2) -> bool {
        let radius = PLAYER_SIZE / 2.0;

        let a = line[0].x - line[1].x;
        let b = line[0].y - line[1].y;
        let x = (a * a + b * b).sqrt();

        (
            (player_pos.x - line[0].x) *
            (line[1].y - line[0].y) -
            (player_pos.y - line[0].y) *
            (line[1].x - line[0].x)
        ).abs() / x <= radius
    }

    fn side_vec_from_usize(position: &Vec2, side: usize) -> Option<[Vec2; 2]> {
        
        match side {
            0 => Some([
                Vec2::new(position.x, position.y),
                Vec2::new(position.x + 1.0, position.y),
            ]),
            3 => Some([
                Vec2::new(position.x + 1.0, position.y),
                Vec2::new(position.x + 1.0, position.y - 1.0),
            ]),
            2 => Some([
                Vec2::new(position.x, position.y + 1.0),
                Vec2::new(position.x + 1.0, position.y + 1.0),
            ]),
            1 => Some([
                Vec2::new(position.x, position.y),
                Vec2::new(position.x, position.y - 1.0),
            ]),
            _ => None
        }
    }

    fn corner_from_usize(position: &Vec2, side: usize) -> Option<Vec2> {
        
        match side {
            0 => Some(
                Vec2::new(position.x, position.y)
            ),
            1 => Some(
                Vec2::new(position.x, position.y + 1.0)
            ),
            2 => Some(
                Vec2::new(position.x + 1.0, position.y + 1.0)
            ),
            3 => Some(
                Vec2::new(position.x + 1.0, position.y)
            ),
            _ => None
        }
    }
}
