mod minimap;
mod pixels;
mod utils;

use crate::game::minimap::Minimap;
use crate::game::pixels::Pixels;
use crate::game::utils::*;
use crate::program::state::ProgramState;
use common::map::Map;
use hecs::Entity;
use notan::app::{App, Color, Graphics, Plugins};
use notan::draw::{CreateDraw, DrawImages, DrawShapes, DrawTransform};
use notan::egui::{DragValue, EguiPluginSugar, Grid, Slider, Ui, Widget, Window};
use notan::prelude::*;
use std::f32::consts::PI;
use std::fmt::{Display, Formatter};

const PLAYER_SPEED: f32 = 0.1;
const CAMERA_SENSITIVITY: f32 = 0.1;

pub struct Game {
    world: hecs::World,
    map: Map,

    pixels: Pixels,
    minimap: Minimap,

    player: Entity,
}

#[derive(Debug)]
pub struct Player {
    x: f32,
    dx: f32,
    y: f32,
    dy: f32,
    a: f32, // angle
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
        let a = 90.0;
        let player = world.spawn((Player {
            x: 1.5,
            dx: deg_to_rad(a).cos(),
            y: 1.5,
            dy: -deg_to_rad(a).sin(),
            a,
        },));

        Game {
            world,
            map,
            player,

            pixels,
            minimap,
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
        let p = self
            .world
            .query_one_mut::<&mut Player>(self.player)
            .unwrap();
        let w = self.map.get_width() as f32;
        let h = self.map.get_height() as f32;

        if app.keyboard.is_down(KeyCode::W) {
            if p.x + p.dx * PLAYER_SPEED <= 0.0 {
                p.x = 0.0;
            } else if p.x + p.dx * PLAYER_SPEED >= w as f32 {
                p.x = w;
            } else {
                p.x += p.dx * PLAYER_SPEED;
            }

            if p.y + p.dy * PLAYER_SPEED <= 0.0 {
                p.y = 0.0;
            } else if p.y + p.dy * PLAYER_SPEED >= h as f32 {
                p.y = h;
            } else {
                p.y += p.dy * PLAYER_SPEED;
            }
        }

        if app.keyboard.is_down(KeyCode::A) {
            if p.x + p.dy * PLAYER_SPEED <= 0.0 {
                p.x = 0.0;
            } else if p.x + p.dy * PLAYER_SPEED >= w as f32 {
                p.x = w;
            } else {
                p.x += p.dy * PLAYER_SPEED;
            }

            if p.y - p.dx * PLAYER_SPEED <= 0.0 {
                p.y = 0.0;
            } else if p.y - p.dx * PLAYER_SPEED >= h as f32 {
                p.y = h;
            } else {
                p.y -= p.dx * PLAYER_SPEED;
            }
        }

        if app.keyboard.is_down(KeyCode::S) {
            if p.x - p.dx * PLAYER_SPEED <= 0.0 {
                p.x = 0.0;
            } else if p.x - p.dx * PLAYER_SPEED >= w as f32 {
                p.x = w;
            } else {
                p.x -= p.dx * PLAYER_SPEED;
            }

            if p.y - p.dy * PLAYER_SPEED <= 0.0 {
                p.y = 0.0;
            } else if p.y - p.dy * PLAYER_SPEED >= h as f32 {
                p.y = h;
            } else {
                p.y -= p.dy * PLAYER_SPEED;
            }
        }

        if app.keyboard.is_down(KeyCode::D) {
            if p.x - p.dy * PLAYER_SPEED <= 0.0 {
                p.x = 0.0;
            } else if p.x - p.dy * PLAYER_SPEED >= w as f32 {
                p.x = w;
            } else {
                p.x -= p.dy * PLAYER_SPEED;
            }

            if p.y + p.dx * PLAYER_SPEED <= 0.0 {
                p.y = 0.0;
            } else if p.y + p.dx * PLAYER_SPEED >= h as f32 {
                p.y = h;
            } else {
                p.y += p.dx * PLAYER_SPEED;
            }
        }

        if app.keyboard.is_down(KeyCode::Left) {
            p.a += CAMERA_SENSITIVITY;
            p.a = fix_angle(p.a);
            p.dx = deg_to_rad(p.a).cos();
            p.dy = -deg_to_rad(p.a).sin();
        }

        if app.keyboard.is_down(KeyCode::Right) {
            p.a -= CAMERA_SENSITIVITY;
            p.a = fix_angle(p.a);
            p.dx = deg_to_rad(p.a).cos();
            p.dy = -deg_to_rad(p.a).sin();
        }
    }

    fn draw(
        &mut self,
        app: &mut App,
        assets: &mut Assets,
        gfx: &mut Graphics,
        plugins: &mut Plugins,
    ) {
        let p = self
            .world
            .query_one_mut::<&mut Player>(self.player)
            .unwrap();
        let m = self.map.clone();

        // Draw a red dot
        let x = p.x;
        let y = p.y;
        self.pixels.set_color(x as usize, y as usize, Color::RED);

        // Draw canvas background
        let (width, height) = self.pixels.dimensions();

        // init map
        //
        let map_w = m.get_width();
        let map_h = m.get_height();

        // Render pixels
        self.pixels.flush(gfx);
        self.pixels.clear(Color::BLACK);

        let mut draw = gfx.create_draw();
        draw.image(self.pixels.texture()).scale(1.0, 1.0);
        // Render map
        let rect_w = (width / (map_w * 2)) as f32;
        let rect_h = (height / map_h) as f32;
        for j in 0..map_h {
            // draw the map
            for i in 0..map_w {
                match self.map.cell(i, j) {
                    common::map::MapCell::Empty => {}
                    common::map::MapCell::Wall(wall_color) => {
                        let rect_x = (i as f32) * rect_w;
                        let rect_y = (j as f32) * rect_h;
                        draw.rect((rect_x, rect_y), (rect_w, rect_h));
                    }
                }
            }
        }

        const FOV: f32 = PI / 3.;
        draw.rect((p.x * rect_w - 5., p.y * rect_h - 5.), (5., 5.));

        // DRAW FOV RAYCAST
        for i in 0..(width / 2) {
            let mut t = 0.;
            // draw the visibility cone
            let angle = p.a - FOV / 2. + FOV * i as f32 / (width as f32 / 2.);

            while t < 20. {
                let cx = p.x + t * angle.cos();
                let cy = p.y + t * angle.sin();
                match self.map.cell(cx as usize, cy as usize) {
                    common::map::MapCell::Wall(wall_color) => {
                        let column_height = height as f32 / t;
                        draw.rect((width as f32 / 2.0 + i as f32, 1.0), (1.0, column_height)).fill_color(Color::new(1. -(t/10.), 1. -(t/10.), 1. -(t/10.), 1.));;
                        break
                    },
                    common::map::MapCell::Empty => {
                        let pix_x = cx * rect_w;
                        let pix_y = cy * rect_h;

                        self.pixels
                            .set_color(pix_x as usize, pix_y as usize, Color::WHITE)
                    }
                }
                t = t + 0.05;
            }
        }

        // Render pixels
        self.pixels.flush(gfx);
        self.pixels.clear(Color::BLACK);

        // Drawing minimap
        self.minimap.draw(&mut draw, width, height);

        gfx.render(&draw);

        // Render egui
        let out = plugins.egui(|ctx| {
            Window::new("Debug")
                .collapsible(true)
                .resizable(false)
                .show(ctx, |ui| self.debug_ui(ui));
        });

        gfx.render(&out);
    }
}

impl Game {
    fn debug_ui(&mut self, ui: &mut Ui) {
        Grid::new("debug_grid_1").show(ui, |ui| {
            let p = self
                .world
                .query_one_mut::<&mut Player>(self.player)
                .unwrap();
            let (width, height) = self.pixels.dimensions();

            ui.label("X");
            Slider::new(&mut p.x, 0.0..=width as f32 - 1.0).ui(ui);
            ui.end_row();

            ui.label("Y");
            Slider::new(&mut p.y, 0.0..=height as f32 - 1.0).ui(ui);
            ui.end_row();
        });
    }
}
