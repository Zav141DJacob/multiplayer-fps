use glam::{IVec2, Vec2};
use notan::prelude::Color;
use common::map::{Map, MapCell, Wall};
use crate::game::raycast::{MAX_VIEW_DISTANCE, Perspective, RayCaster};
use crate::game::texture::draw_column::DrawColumn;
use crate::game::texture::get_wall_texture;
use crate::game::texture::pixels::Pixels;

struct Hit {
    t: f32,
    pos: Vec2,
    cell: MapCell,
    side: HitSide,
}

enum HitSide {
    Top,
    Bottom,
    Left,
    Right,
}

impl RayCaster {
    pub fn draw_walls(
        &mut self,
        pixels: &mut Pixels,
        camera_pos: Vec2,
        camera_dir: Vec2,
        perspective: Perspective,
        map: &Map,
    ) {
        puffin::profile_function!();
        self.minimap_rays.clear();
        self.depth_map.clear();

        // DRAW FOV RAYCAST
        for (ray_dir, column) in self.ray_gen.iter(camera_dir).zip(pixels.column_iter_mut()) {

            let hit = match ray_algorithm(camera_pos, ray_dir, map) {
                None => {
                    // No hit, register max distance instead
                    self.minimap_rays.push(camera_pos + ray_dir * MAX_VIEW_DISTANCE);
                    self.depth_map.push(MAX_VIEW_DISTANCE);
                    continue
                }
                Some(hit) => {
                    self.minimap_rays.push(hit.pos);
                    self.depth_map.push(hit.t);
                    hit
                },
            };

            // Hit confirmed beyond this point

            let wall_height = self.proj_dist / (hit.t * ray_dir.dot(camera_dir));

            let set_color_fn = |current: &mut [u8; 4], new: [u8; 4]|  {
                *current = if matches!(hit.side, HitSide::Left | HitSide::Right) {
                    const DARKEN: u16 = (0.8 * 256.0) as u16;
                    [
                        (new[0] as u16 * DARKEN / 256) as u8,
                        (new[1] as u16 * DARKEN / 256) as u8,
                        (new[2] as u16 * DARKEN / 256) as u8,
                        new[3]
                    ]
                } else {
                    new
                }
            };

            let wall = match hit.cell {
                MapCell::Empty => unreachable!(),
                MapCell::Wall(wall) => wall,
            };

            match wall {
                Wall::SolidColor(wall_color) => {
                    Color::from(wall_color)
                        .draw_column(
                            column,
                            0.0,
                            wall_height,
                            perspective,
                            set_color_fn,
                        );
                }
                Wall::Textured(wall_type) => {
                    let texture = get_wall_texture(wall_type);

                    // Calculate how far along the wall we are.
                    let wall_x = match hit.side {
                        HitSide::Top => 1.0 - hit.pos.x.fract(),
                        HitSide::Bottom => hit.pos.x.fract(),
                        HitSide::Left => hit.pos.y.fract(),
                        HitSide::Right => 1.0 - hit.pos.y.fract(),
                    };

                    texture.draw_column(
                        column,
                        wall_x,
                        wall_height,
                        perspective,
                        set_color_fn,
                    );
                }
            }
        }

    }
}

fn ray_algorithm(
    ray_start: Vec2,
    ray_dir: Vec2,
    map: &Map,
) -> Option<Hit> {
    puffin::profile_function!();
    let ray_unit_step_size = Vec2::new(
        (1.0 + (ray_dir.y / ray_dir.x) * (ray_dir.y / ray_dir.x)).sqrt(),
        (1.0 + (ray_dir.x / ray_dir.y) * (ray_dir.x / ray_dir.y)).sqrt(),
    );

    let mut map_check = IVec2::new(ray_start.x.floor() as i32, ray_start.y.floor() as i32);
    let mut ray_length_1d = Vec2::default();

    let mut map_step = IVec2::new(0, 0);

    if ray_dir.x < 0.0 {
        map_step.x = -1;
        ray_length_1d.x = (ray_start.x - map_check.x as f32) * ray_unit_step_size.x;
    } else {
        map_step.x = 1;
        ray_length_1d.x = ((map_check.x + 1) as f32 - ray_start.x) * ray_unit_step_size.x;
    }

    if ray_dir.y < 0.0 {
        map_step.y = -1;
        ray_length_1d.y = (ray_start.y - map_check.y as f32) * ray_unit_step_size.y;
    } else {
        map_step.y = 1;
        ray_length_1d.y = ((map_check.y + 1) as f32 - ray_start.y) * ray_unit_step_size.y;
    }

    let mut f_distance = 0.0;

    while f_distance <= MAX_VIEW_DISTANCE {
        let check_vertical = ray_length_1d.x < ray_length_1d.y;
        if check_vertical {
            map_check.x += map_step.x;
            f_distance = ray_length_1d.x;
            ray_length_1d.x += ray_unit_step_size.x;
        } else {
            map_check.y += map_step.y;
            f_distance = ray_length_1d.y;
            ray_length_1d.y += ray_unit_step_size.y;
        }

        if map_check.x < 0
            || map_check.x >= map.width as i32
            || map_check.y < 0
            || map_check.y >= map.height as i32
        {
            continue;
        }

        let pos = ray_start + ray_dir * f_distance;

        let cell = map.cell(map_check.x, map_check.y);

        let cell = match cell {
            MapCell::Empty => continue,
            cell => cell,
        };

        let side = if check_vertical {
            if ray_dir.x > 0.0 {
                HitSide::Left
            } else {
                HitSide::Right
            }
        } else {
            #[allow(clippy::collapsible_else_if)]
            if ray_dir.y < 0.0 {
                HitSide::Bottom
            } else {
                HitSide::Top
            }
        };

        return Some(Hit {
            t: f_distance,
            pos,
            cell,
            side,
        });
    }

    None
}
