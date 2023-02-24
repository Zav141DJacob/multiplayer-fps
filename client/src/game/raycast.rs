use crate::game::*;
use notan::prelude::Color;

pub fn draw_walls(
    pixels: &mut Pixels,
    width: usize,
    height: usize,
    p: (&Position, &Direction),
    map: Map,
    texture: &textures::Texture,
    minimap_rays: &mut Vec<Vec2>,
    depth_map: &mut [f32],
) {
    const FOV: f32 = PI / 3.;

    // DRAW FOV RAYCAST
    for i in 0..width {
        // draw the visibility cone
        let angle = -p.1.xy.angle_between(Vec2::X) - FOV / 2. + FOV * i as f32 / width as f32;
        let angle_vec2 = Vec2::from_angle(angle);

        let ray_start = p.0.xy;

        let ray_unit_step_size = Vec2::new(
            (1.0 + (angle_vec2.y / angle_vec2.x) * (angle_vec2.y / angle_vec2.x)).sqrt(),
            (1.0 + (angle_vec2.x / angle_vec2.y) * (angle_vec2.x / angle_vec2.y)).sqrt(),
        );

        let mut map_check = glam::i32::ivec2(ray_start.x as i32, ray_start.y as i32);
        let mut ray_length_1d = Vec2::default();

        let mut step = glam::i32::ivec2(0, 0);

        if angle_vec2.x < 0.0 {
            step.x = -1;
            ray_length_1d.x = (ray_start.x - map_check.x as f32) * ray_unit_step_size.x;
        } else {
            step.x = 1;
            ray_length_1d.x = ((map_check.x + 1) as f32 - ray_start.x) * ray_unit_step_size.x;
        }

        if angle_vec2.y < 0.0 {
            step.y = -1;
            ray_length_1d.y = (ray_start.y - map_check.y as f32) * ray_unit_step_size.y;
        } else {
            step.y = 1;
            ray_length_1d.y = ((map_check.y + 1) as f32 - ray_start.y) * ray_unit_step_size.y;
        }

        let mut f_distance = 0.0;

        while f_distance < 20. {
            if ray_length_1d.x < ray_length_1d.y {
                map_check.x += step.x;
                f_distance = ray_length_1d.x;
                ray_length_1d.x += ray_unit_step_size.x;
            } else {
                map_check.y += step.y;
                f_distance = ray_length_1d.y;
                ray_length_1d.y += ray_unit_step_size.y;
            }

            if map_check.x >= 0
                && map_check.x < map.width as i32
                && map_check.y >= 0
                && map_check.y < map.height as i32
            {
                let intersection = ray_start + angle_vec2 * f_distance;

                //println!("{} {}", intersection.x, intersection.y);
                let cx = intersection.x;
                let cy = intersection.y;
                match map.cell(map_check.x as usize, map_check.y as usize) {
                    common::map::MapCell::Wall(Wall::SolidColor(wall_color)) => {
                        let column_height = height as f32
                            / (f_distance * (angle - -p.1.xy.angle_between(Vec2::X)).cos());
                        for o in 0..1 {
                            for p in 0..column_height as usize {
                                let y = height as f32 / 2. - column_height / 2.;
                                let rx = i + o;
                                let ry = y as usize + p;
                                if rx >= width || ry >= height {
                                    continue;
                                } // no need to check negative values, (unsigned variables)
                                pixels.set_color(
                                    rx,
                                    ry,
                                    Color::new(
                                        wall_color[1] - (f_distance / 20.),
                                        wall_color[1] - (f_distance / 20.),
                                        wall_color[1] - (f_distance / 20.),
                                        1.,
                                    ),
                                );
                            }
                        }
                        minimap_rays.push(Vec2::new(cx, cy));
                        break;
                    }
                    common::map::MapCell::Wall(Wall::Textured(wall_type)) => {
                        let column_height = height as f32
                            / (f_distance * (angle - -p.1.xy.angle_between(Vec2::X)).cos());
                        let hitx = cx - (cx + 0.5).floor();
                        let hity = cy - (cy + 0.5).floor();
                        let mut x_texcoord = hitx * texture.size as f32;

                        if hity.abs() > hitx.abs() {
                            x_texcoord = hity * texture.size as f32;
                        }
                        if x_texcoord < 0. {
                            x_texcoord = x_texcoord.abs()
                        }
                        let column = texture.texture_column(
                            wall_type as i32,
                            x_texcoord,
                            column_height,
                        );

                        depth_map[i] = f_distance;
                        for j in 0..column_height as usize {
                            let y = height as f32 / 2. - column_height / 2.;
                            let rx = i;
                            let ry = j + y as usize;
                            if rx >= height || ry >= width {
                                continue;
                            } // no need to check negative values, (unsigned variables)
                            let mut color = column[j];
                            if column_height > height as f32 {
                                let x = (column_height - height as f32) as usize / 2;
                                color = column[x + j];
                            }
                            pixels.set_color(
                                rx,
                                ry,
                                Color::new(
                                    color.r - (f_distance / 20.),
                                    color.g - (f_distance / 20.),
                                    color.b - (f_distance / 20.),
                                    color.a,
                                ),
                            );
                        }
                        minimap_rays.push(Vec2::new(cx, cy));
                        break;
                    }
                    common::map::MapCell::Empty => {}
                }
            }
        }
    }
}

pub fn draw_enemies(
    pixels: &mut Pixels,
    width: usize,
    height: usize,
    p: (&Position, &Direction),
    enemies: &[enemy::Sprite],
    enemy_texture: &textures::Texture,
    depth_map: &mut [f32],
) {
    const FOV: f32 = PI / 3.;
    for enemy in enemies.iter() {
        let mut sprite_dir = (enemy.position.xy.y - p.0.xy.y).atan2(enemy.position.xy.x - p.0.xy.x);
        let sprite_dist = ((p.0.xy.x - enemy.position.xy.x).powf(2.)
            + (p.0.xy.y - enemy.position.xy.y).powf(2.))
        .sqrt();
        while sprite_dir - -p.1.xy.angle_between(Vec2::X) > PI {
            sprite_dir -= 2. * PI; // remove unncesessary periods from the relative direction
        }
        while sprite_dir - -p.1.xy.angle_between(Vec2::X) < -PI {
            sprite_dir += 2. * PI;
        }
        let sprite_screen_size = 2000_f32.min(height as f32 / sprite_dist);
        let h_offset = (sprite_dir - -p.1.xy.angle_between(Vec2::X)) / FOV * width as f32
            + width as f32 / 2.
            - (enemy_texture.size as f32 * 5. / sprite_dist);
        let v_offset = height as i32 / 2 - sprite_screen_size as i32 / 2;

        for i in 0..sprite_screen_size as i32 {
            if (h_offset + i as f32) < 0. || (h_offset + i as f32) >= width as f32 {
                continue;
            }
            if depth_map.len() < h_offset as usize + i as usize
                || depth_map[(h_offset + i as f32) as usize] < sprite_dist
            {
                continue;
            }; // this sprite column is occluded
            for j in 0..sprite_screen_size as i32 {
                if v_offset < 0 || (v_offset + j) < 0 || (v_offset + j) >= height as i32 {
                    continue;
                }
                let mut r = enemy_texture.get(
                    i * enemy_texture.size as i32 / sprite_screen_size as i32,
                    j * enemy_texture.size as i32 / sprite_screen_size as i32,
                    enemy.texture_id,
                );
                if r.a == 0. {
                    continue;
                }
                r.r -= sprite_dist / 17.;
                r.b -= sprite_dist / 17.;
                r.g -= sprite_dist / 17.;
                pixels.set_color(
                    (h_offset + i as f32) as usize,
                    v_offset as usize + j as usize,
                    r,
                );
            }
        }
    }
}
