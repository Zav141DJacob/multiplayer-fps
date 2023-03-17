use crate::ecs::systems::ServerSystems;
use crate::ecs::ServerEcs;
use common::defaults::PLAYER_SIZE;
use common::ecs::components::{Position, Player, Bullet, WithId, Health, Damage, ShotBy};
use common::map::{Map, MapCell};
use glam::Vec2;
use hecs::Entity;
trait WallCollision {
    fn prepare_wall_collisions(ecs: &mut ServerEcs);
    fn get_sides(map: &Map, x_f: f32, y_f: f32, x_i: i32, y_i: i32) -> Vec<(Vec2, MapCell)>{
        let mut return_vec: Vec<(Vec2, MapCell)> = Vec::new();
            return_vec.push(
                (Vec2::new(x_f, y_f + 1.0), map.cell(x_i, y_i + 1))
            );

            return_vec.push(
                (Vec2::new(x_f + 1.0, y_f), map.cell(x_i + 1, y_i))
            );

            return_vec.push(
                (Vec2::new(x_f, y_f - 1.0), map.cell(x_i, y_i - 1))
            );
        
            return_vec.push(
                (Vec2::new(x_f - 1.0, y_f), map.cell(x_i - 1, y_i))
            );
        return_vec
    }

    fn get_corners(map: &Map, x_f: f32, y_f: f32, x_i: i32, y_i: i32) -> Vec<(Vec2, MapCell)> {
        let mut return_vec: Vec<(Vec2, MapCell)> = Vec::new();
            return_vec.push(
                (Vec2::new(x_f + 1.0, y_f + 1.0), map.cell(x_i + 1, y_i + 1))
            );

            return_vec.push(
                (Vec2::new(x_f + 1.0, y_f - 1.0), map.cell(x_i + 1, y_i - 1))
            );

            return_vec.push(
                (Vec2::new(x_f - 1.0, y_f - 1.0), map.cell(x_i - 1, y_i - 1))
            );
        
            return_vec.push(
                (Vec2::new(x_f - 1.0, y_f + 1.0), map.cell(x_i - 1, y_i + 1))
            );
        return_vec
    }

    //  Checks if a line is inside a circle
    // takes in the line and a position as a Vec2
    // https://math.stackexchange.com/questions/275529/check-if-line-intersects-with-circles-perimeter
    fn in_circle(line: &[Vec2; 2], player_pos: &Vec2) -> bool {
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
    fn shared_logic(map: Map, pos: &Vec2, size: f32) -> Vec2 {
        let mut to_pos = pos.clone();

        let x_floored_int = pos.x.floor() as i32;
        let y_floored_int = pos.y.floor() as i32;

        let x_floored_f = pos.x.floor();
        let y_floored_f = pos.y.floor();

        let sides: Vec<(Vec2, MapCell)> = Self::get_sides(
            &map, x_floored_f, 
            y_floored_f, 
            x_floored_int, 
            y_floored_int
        );

        let corners: Vec<(Vec2, MapCell)> = Self::get_corners(
            &map, x_floored_f, 
            y_floored_f, 
            x_floored_int, 
            y_floored_int
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
                            },
                            3 => {
                                to_pos.x = x_floored_f + (size / 2.0);
                            },
                            2 => {
                                to_pos.y = y_floored_f + (size / 2.0);
                            },
                            1 => {
                                to_pos.x = x_floored_f + (1.0 - size / 2.0); 
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
                        if !(-1.0..=1.0).contains(&to_acos) {
                            to_acos = to_acos - to_acos % 1.0;
                        }

                        let alpha = 360.0 - 90.0 - to_acos.acos();


                        let determinant = (2.0 * d * alpha.cos()).powf(2.0) + 4.0 * (r.powf(2.0) - d.powf(2.0));

                        let x_1 = 
                        (
                            2.0 * d * alpha.cos() + determinant.sqrt()
                        ) / 2.0; 
                        let x_2 = 
                        (
                            2.0 * d * alpha.cos() - determinant.sqrt()
                        ) / 2.0; 

                        let x_max = x_1.max(x_2);
                        let change_x: f32;

                        change_x = x_max;
                        match i {
                            0 => {
                                to_pos.y -= change_y;
                                to_pos.x -= change_x;
                            }
                            1 => {
                                to_pos.y += change_y;
                                to_pos.x -= change_x;
                            }
                            2 => {
                                to_pos.y += change_y;
                                to_pos.x += change_x;
                            }

                            // done, dont change
                            3 => {
                                to_pos.y -= change_y;
                                to_pos.x += change_x;
                            }
                            _ => panic!("AAAAAAAAAAAAAAA"),
                        }
                    }
                }
            }
        }
        to_pos
      // implement shared logic of the functions in here
    }
}

impl WallCollision for Player {
    fn prepare_wall_collisions(ecs: &mut ServerEcs) {
        let map = ecs.resources.get::<Map>().unwrap().clone();
        // let mut player_positions: Vec<(T, Health, Vec2)> = Vec::new();
        let mut to_remove: Vec<Entity> = Vec::new();


        let mut bullet_positions: Vec<(Entity, Bullet, Vec2, Damage)> = Vec::new();


        {
            let bullet_query = ecs.world.query_mut::<(&Bullet, &Position, &Damage)>();
            for (entity, (bullet, bullet_pos, damage)) in bullet_query {
                bullet_positions.push((entity, *bullet, bullet_pos.0, *damage));
            }
        }
        let query = ecs.world.query_mut::<(&Player, &mut Health, &mut Position, &mut ShotBy)>();

        for (entity, (player, health, pos, shot_by)) in query {
            
            let to_pos = Self::shared_logic(map.clone(), &pos.0, PLAYER_SIZE);

            {
                let mut pos = ecs.observer.observe_component(entity, pos);
                let pos = &mut pos.0;


                *pos = to_pos;
            }
            {
                
                for (bullet_entity, bullet, bullet_pos, damage) in &bullet_positions {
                    if bullet_pos.distance(to_pos) < PLAYER_SIZE / 2.0 {

                        if player.id() != bullet.id() {
                            to_remove.push(*bullet_entity);

                            {
                                //ToDo
                                //  add this code into another function (ie. do_damage())

                                let mut health = ecs.observer.observe_component(entity, health);
                                let health = &mut health.0;
                                *health -= damage.0 as u32;
                            }
                            {
                                let mut shot_by = ecs.observer.observe_component(entity, shot_by);
                                let shot_by = &mut shot_by.id;
                                *shot_by = Some(bullet.id());

                            }
                        }
                    }
                }
            }
        }
        for e in to_remove {
            ecs.observed_world().despawn(e).unwrap();
        }
    }
} 
impl WallCollision for Bullet {
    fn prepare_wall_collisions(ecs: &mut ServerEcs) {
        let map = ecs.resources.get::<Map>().unwrap().clone();
        // let mut player_positions: Vec<(T, Health, Vec2)> = Vec::new();
        let mut to_remove: Vec<Entity> = Vec::new();

        let query = ecs.world.query_mut::<(&Bullet, &mut Position)>();

        for (entity, (_, pos)) in query {
            
            let to_pos = Self::shared_logic(map.clone(), &pos.0, 0.0);

            if pos.0 != to_pos {
                to_remove.push(entity);
            }
        }
        for e in to_remove {
            ecs.observed_world().despawn(e).unwrap();
        }
    }
}
impl ServerSystems {
    pub fn collision_system(ecs: &mut ServerEcs, _dt: f32) {
        Player::prepare_wall_collisions(ecs);
        Bullet::prepare_wall_collisions(ecs);
    }
    
}
