mod ecs;
mod enemy;
mod minimap;
mod pixels;
mod textures;

use crate::game::enemy::*;
use crate::game::minimap::Minimap;
use crate::game::pixels::Pixels;
use crate::game::textures::Texture;
use crate::program::state::ProgramState;
use common::map::{Map, Textured, Wall};
use hecs::Entity;
use notan::app::{App, Color, Graphics, Plugins};

use notan::draw::{CreateDraw, DrawImages, DrawShapes, DrawTransform};
use notan::prelude::*;
use std::cmp;
use std::f32::consts::PI;
use std::fmt::{Display, Formatter};

use notan::egui::{DragValue, EguiPluginSugar, Grid, Slider, Ui, Widget, Window};
// use notan::prelude::{Assets, Texture, KeyCode};

use glam::f32::Vec2;
use fps_counter::FPSCounter;

const PLAYER_SPEED: f32 = 0.1;
const CAMERA_SENSITIVITY: f32 = 0.08; // rad

pub struct Game {
    world: hecs::World,
    map: Map,

    pixels: Pixels,
    minimap: Minimap,

    player: Entity,
    texture: Texture,
    enemy_texture: Texture,
    enemies: Vec<enemy::Sprite>,

    fps: FPSCounter,
}

#[derive(Debug)]
pub struct Player;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    xy: Vec2,
}

pub struct Direction {
    xy: Vec2,
}

impl Game {
    pub fn new(gfx: &mut Graphics) -> Self {
        let (width, height) = gfx.size();
        let (width, height) = (width as usize, height as usize);

        let map = Map::default();

        let pixels = Pixels::new(width, height, gfx);
        let mut minimap = Minimap::new(map.clone(), gfx);
        minimap.render_map(gfx);

        let mut world = hecs::World::new();
        let player = world.spawn((
            Player,
            Position {
                xy: Vec2::new(1.5, 1.5),
            },
            Direction {
                xy: Vec2::new(0.0, 1.0),
            },
        ));

        // WIP enemies
        let enemies_init = vec![(3.834, 2.765, 0), (5.323, 5.365, 2), (8.123, 8.265, 3)];
        let mut enemies = vec![];
        for i in enemies_init {
            let enemy = Sprite {
                texture_id: i.2,
                position: Position {
                    xy: Vec2::new(i.0, i.1),
                },
            };
            enemies.push(enemy);
            world.spawn((
                enemy,
                enemy.position,
                Direction {
                    xy: Vec2::new(0.0, 1.0),
                },
            ));
        }

        let texture = Texture::new(include_bytes!("../../assets/walltext.png")).unwrap();

        let enemy_texture = Texture::new(include_bytes!("../../assets/monsters.png")).unwrap();

        let fps = FPSCounter::new();

        Game {
            world,
            map,
            player,

            pixels,
            minimap,
            texture,
            enemies,
            enemy_texture,

            fps,
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Game")
    }
}

impl ProgramState for Game {
    fn update(&mut self, app: &mut App, assets: &mut Assets, plugins: &mut Plugins) {
        let mut p = self
            .world
            .query_one_mut::<(&mut Position, &mut Direction)>(self.player)
            .unwrap();
        let w = self.map.get_width() as f32;
        let h = self.map.get_height() as f32;

        if app.keyboard.is_down(KeyCode::W) {
            if (p.0.xy + p.1.xy * PLAYER_SPEED).x < 0.0 {
                p.0.xy.x = 0.0;
            } else if (p.0.xy + p.1.xy * PLAYER_SPEED).y < 0.0 {
                p.0.xy.y = 0.0;
            } else if (p.0.xy + p.1.xy * PLAYER_SPEED).x > w as f32 {
                p.0.xy.x = w;
            } else if (p.0.xy + p.1.xy * PLAYER_SPEED).y > h as f32 {
                p.0.xy.y = h;
            } else {
                p.0.xy += p.1.xy * PLAYER_SPEED;
            }
        }

        if app.keyboard.is_down(KeyCode::A) {
            if (p.0.xy - p.1.xy.perp() * PLAYER_SPEED).x < 0.0 {
                p.0.xy.x = 0.0;
            } else if (p.0.xy - p.1.xy.perp() * PLAYER_SPEED).y < 0.0 {
                p.0.xy.y = 0.0;
            } else if (p.0.xy - p.1.xy.perp() * PLAYER_SPEED).x > w as f32 {
                p.0.xy.x = w;
            } else if (p.0.xy - p.1.xy.perp() * PLAYER_SPEED).y > h as f32 {
                p.0.xy.y = h;
            } else {
                p.0.xy -= p.1.xy.perp() * PLAYER_SPEED;
            }
        }

        if app.keyboard.is_down(KeyCode::S) {
            if (p.0.xy - p.1.xy * PLAYER_SPEED).x < 0.0 {
                p.0.xy.x = 0.0;
            } else if (p.0.xy - p.1.xy * PLAYER_SPEED).y < 0.0 {
                p.0.xy.y = 0.0;
            } else if (p.0.xy - p.1.xy * PLAYER_SPEED).x > w as f32 {
                p.0.xy.x = w;
            } else if (p.0.xy - p.1.xy * PLAYER_SPEED).y > h as f32 {
                p.0.xy.y = h;
            } else {
                p.0.xy -= p.1.xy * PLAYER_SPEED;
            }
        }

        if app.keyboard.is_down(KeyCode::D) {
            if (p.0.xy + p.1.xy.perp() * PLAYER_SPEED).x < 0.0 {
                p.0.xy.x = 0.0;
            } else if (p.0.xy + p.1.xy.perp() * PLAYER_SPEED).y < 0.0 {
                p.0.xy.y = 0.0;
            } else if (p.0.xy + p.1.xy.perp() * PLAYER_SPEED).x > w as f32 {
                p.0.xy.x = w;
            } else if (p.0.xy + p.1.xy.perp() * PLAYER_SPEED).y > h as f32 {
                p.0.xy.y = h;
            } else {
                p.0.xy += p.1.xy.perp() * PLAYER_SPEED;
            }
        }

        if app.keyboard.is_down(KeyCode::Left) {
            p.1.xy = p.1.xy.rotate(Vec2::from_angle(-CAMERA_SENSITIVITY));
        }

        if app.keyboard.is_down(KeyCode::Right) {
            p.1.xy = p.1.xy.rotate(Vec2::from_angle(CAMERA_SENSITIVITY));
        }
    }

    fn draw(
        &mut self,
        app: &mut App,
        assets: &mut Assets,
        gfx: &mut Graphics,
        plugins: &mut Plugins,
    ) {
        let mut query = self
            .world
            .query_one::<(&Position, &Direction)>(self.player)
            .unwrap();
        let p = query.get().unwrap();

        // Draw canvas background
        let (width, height) = self.pixels.dimensions();

        // init map
        //

        // Render pixels
        self.pixels.flush(gfx);
        self.pixels.clear(Color::BLACK);

        let mut draw = gfx.create_draw();
        draw.image(self.pixels.texture()).scale(1.0, 1.0);

        // Render map

        const FOV: f32 = PI / 3.;
        let mut depth_map = vec![0.; width];

        let mut minimap_rays = Vec::new();

        // DRAW FOV RAYCAST
        for i in 0..width {
            let mut t = 0.;
            // draw the visibility cone
            let angle = -p.1.xy.angle_between(Vec2::X) - FOV / 2. + FOV * i as f32 / width as f32;

            while t < 20. {
                let cx = p.0.xy.x + t * angle.cos();
                let cy = p.0.xy.y + t * angle.sin();
                match self.map.cell(cx as usize, cy as usize) {
                    common::map::MapCell::Wall(Wall::SolidColor(wall_color)) => {
                        let column_height =
                            height as f32 / (t * (angle - -p.1.xy.angle_between(Vec2::X)).cos());
                        for o in 0..1 {
                            for p in 0..column_height as usize {
                                let y = height as f32 / 2. - column_height / 2.;
                                let rx = i + o;
                                let ry = y as usize + p;
                                if (rx >= width || ry >= height) {
                                    continue;
                                } // no need to check negative values, (unsigned variables)
                                self.pixels.set_color(
                                    rx,
                                    ry,
                                    Color::new(
                                        wall_color[1] - (t / 20.),
                                        wall_color[1] - (t / 20.),
                                        wall_color[1] - (t / 20.),
                                        1.,
                                    ),
                                );
                            }
                        }
                        minimap_rays.push(Vec2::new(cx, cy));
                        break;
                    }
                    common::map::MapCell::Wall(Wall::Textured(wall_type)) => {
                        let column_height =
                            height as f32 / (t * (angle - -p.1.xy.angle_between(Vec2::X)).cos());
                        let hitx = cx - (cx + 0.5).floor();
                        let hity = cy - (cy + 0.5).floor();
                        let mut x_texcoord = hitx * self.texture.size as f32;

                        if (hity.abs() > hitx.abs()) {
                            x_texcoord = hity * self.texture.size as f32;
                        }
                        if (x_texcoord < 0.) {
                            x_texcoord = x_texcoord.abs()
                        }
                        let column = self.texture.texture_column(
                            wall_type as i32,
                            x_texcoord,
                            column_height,
                        );

                        depth_map[i] = t;
                        for j in 0..column_height as usize {
                            let y = height as f32 / 2. - column_height / 2.;
                            let rx = i;
                            let ry = j + y as usize;
                            if (rx >= height || ry >= width) {
                                continue;
                            } // no need to check negative values, (unsigned variables)
                            let color = column[j];
                            self.pixels.set_color(
                                rx,
                                ry,
                                Color::new(
                                    color.r - (t / 20.),
                                    color.g - (t / 20.),
                                    color.b - (t / 20.),
                                    color.a,
                                ),
                            );
                        }
                        minimap_rays.push(Vec2::new(cx, cy));
                        break;
                    }
                    common::map::MapCell::Empty => {}
                }
                t = t + 0.05;
            }
        }
        for (i, enemy) in self.enemies.iter().enumerate() {
            let mut sprite_dir = (enemy.position.xy.y - p.0.xy.y).atan2(enemy.position.xy.x - p.0.xy.x);
            let sprite_dist = ((p.0.xy.x - enemy.position.xy.x).powf(2.)
                + (p.0.xy.y - enemy.position.xy.y).powf(2.))
            .sqrt();
            while (sprite_dir - -p.1.xy.angle_between(Vec2::X) > PI) {
                sprite_dir -= 2. * PI; // remove unncesessary periods from the relative direction
            }
            while (sprite_dir - -p.1.xy.angle_between(Vec2::X) < -PI) {
                sprite_dir += 2. * PI;
            }
            let sprite_screen_size = (2000 as f32).min(height as f32 / sprite_dist);
            let h_offset = (sprite_dir - -p.1.xy.angle_between(Vec2::X)) / FOV * width as f32
                + width as f32 / 2.
                - (self.enemy_texture.size as f32 * 5. / sprite_dist);
            let v_offset = height as i32 / 2 - sprite_screen_size as i32 / 2;

            for i in 0..sprite_screen_size as i32 {
                if (h_offset + i as f32) < 0. || (h_offset + i as f32) >= width as f32 {
                    continue;
                }
                if (depth_map.len() < h_offset as usize + i as usize
                    || depth_map[(h_offset + i as f32) as usize] < sprite_dist)
                {
                    continue;
                }; // this sprite column is occluded
                for j in 0..sprite_screen_size as i32 {
                    if v_offset < 0 || (v_offset + j) < 0 || (v_offset + j) >= height as i32 {
                        continue;
                    }
                    let mut r = self.enemy_texture.get(
                        i * self.enemy_texture.size as i32 / sprite_screen_size as i32,
                        j * self.enemy_texture.size as i32 / sprite_screen_size as i32,
                        enemy.texture_id,
                    );
                    if r.a == 0. {
                        continue;
                    }
                    r.r = r.r - (sprite_dist / 17.);
                    r.b = r.b - (sprite_dist / 17.);
                    r.g = r.g - (sprite_dist / 17.);
                    self.pixels.set_color(
                        (h_offset + i as f32) as usize,
                        v_offset as usize + j as usize,
                        r,
                    );
                }
            }
        }

        // Render pixels
        self.pixels.flush(gfx);
        self.pixels.clear(Color::BLACK);

        // Drawing minimap
        self.minimap.draw(&mut draw, width, height);

        self.minimap.render_vision(
            &mut draw,
            width,
            height,
            p.0.xy,
            Color::new(0.2, 0.2, 0.2, 1.0),
            minimap_rays,
        );

        self.minimap
            .render_player_location(&mut draw, width, height, p.0.xy, Color::RED);

        self.minimap.render_entity_location(
            &mut draw,
            width,
            height,
            self.enemies[0].position.xy,
            Color::WHITE,
        );
        self.minimap.render_entity_location(
            &mut draw,
            width,
            height,
            self.enemies[1].position.xy,
            Color::WHITE,
        );
        self.minimap.render_entity_location(
            &mut draw,
            width,
            height,
            self.enemies[2].position.xy,
            Color::WHITE,
        );

        gfx.render(&draw);

        drop(query);

        // Render egui
        let out = plugins.egui(|ctx| {
            Window::new("Debug")
                .collapsible(true)
                .resizable(false)
                .show(ctx, |ui| self.debug_ui(ui));
        });

        gfx.render(&out);

        let fps_counter = plugins.egui(|ctx| {
            notan::egui::Area::new("fps-counter")
                .fixed_pos(notan::egui::pos2(0.0, 0.0))
                .show(ctx, |ui| ui.label("FPS: ".to_owned() + &self.fps.tick().to_string()));
        });

        gfx.render(&fps_counter);
    }
}

impl Game {
    fn debug_ui(&mut self, ui: &mut Ui) {
        Grid::new("debug_grid_1").show(ui, |ui| {
            let p = self
                .world
                .query_one_mut::<&mut Position>(self.player)
                .unwrap();
            let height = self.map.get_height();
            let width = self.map.get_width();

            ui.label("X");
            Slider::new(&mut p.xy.x, 0.0..=width as f32 - 1.0).ui(ui);
            ui.end_row();

            ui.label("Y");
            Slider::new(&mut p.xy.y, 0.0..=height as f32 - 1.0).ui(ui);
            ui.end_row();
        });
    }
}
