mod pixels;
mod minimap;
mod utils;

use std::f32::consts::PI;
use std::fmt::{Display, Formatter};
use hecs::Entity;
use notan::app::{App, Color, Graphics, Plugins};
use notan::draw::{CreateDraw, DrawImages, DrawTransform, DrawShapes};
use notan::egui::{DragValue, EguiPluginSugar, Grid, Slider, Ui, Widget, Window};
use notan::prelude::*;
use common::map::Map;
use crate::game::utils::*;
use crate::game::pixels::Pixels;
use crate::game::minimap::Minimap;
use crate::program::state::ProgramState;

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
        
        let  map = Map::default();

        let pixels = Pixels::new(width, height, gfx);
        let mut minimap = Minimap::new(map.clone(), gfx);
        minimap.render_map(gfx);

        let mut world = hecs::World::new();
        let a = 90.0;
        let player = world.spawn((Player{ x: 1.5, dx: deg_to_rad(a).cos(), y: 1.5, dy: -deg_to_rad(a).sin(), a },));

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
        let p = self.world.query_one_mut::<&mut Player>(self.player).unwrap();
        let w = self.map.get_width() as f32;
        let h = self.map.get_height() as f32;

        if app.keyboard.is_down(KeyCode::W) {

            if p.x + p.dx * 0.1 <= 0.0 {
                p.x = 0.0;
            } else if p.x + p.dx * 0.1 >= w as f32 {
                p.x = w;
            } else {
                p.x += p.dx * 0.1;
            }

            if p.y + p.dy * 0.1 <= 0.0 {
                p.y = 0.0;
            } else if p.y + p.dy * 0.1 >= h as f32 {
                p.y = h;
            } else {
                p.y += p.dy * 0.1;
            }
        }

        if app.keyboard.is_down(KeyCode::A) {
            p.a += 5.0;
            p.a = fix_angle(p.a);
            p.dx = deg_to_rad(p.a).cos();
            p.dy = -deg_to_rad(p.a).sin();
        }

        if app.keyboard.is_down(KeyCode::S) {

            if p.x - p.dx * 0.1 <= 0.0 {
                p.x = 0.0;
            } else if p.x - p.dx * 0.1 >= w as f32 {
                p.x = w;
            } else {
                p.x -= p.dx * 0.1;
            }

            if p.y - p.dy * 0.1 <= 0.0 {
                p.y = 0.0;
            } else if p.y - p.dy * 0.1 >= h as f32 {
                p.y = h;
            } else {
                p.y -= p.dy * 0.1;
            }
        }

        if app.keyboard.is_down(KeyCode::D) {
            p.a -= 5.0;
            p.a = fix_angle(p.a);
            p.dx = deg_to_rad(p.a).cos();
            p.dy = -deg_to_rad(p.a).sin();
        }
    }

    fn draw(&mut self, app: &mut App, assets: &mut Assets, gfx: &mut Graphics, plugins: &mut Plugins) {
        let p = self.world.query_one_mut::<&mut Player>(self.player).unwrap();

        // Draw a red dot
        let x = p.x;
        let y = p.y;
        self.pixels.set_color(x as usize, y as usize, Color::RED);

        // Draw canvas background
        let (width, height) = self.pixels.dimensions();

        // init map
        const MAP_W: usize = 16; // map width
        const MAP_H: usize = 16; // map height
        const MAP: &str = "0000222222220000\
            1              0\
            1      11111   0\
            1     0        0\
            0     0  1110000\
            0     3        0\
            0   10000      0\
            0   0   11100  0\
            0   0   0      0\
            0   0   1  00000\
            0       1      0\
            2       1      0\
            0       0      0\
            0 0000000      0\
            0              0\
            0002222222200000";

        // Render pixels
        self.pixels.flush(gfx);
        self.pixels.clear(Color::BLACK);

        let mut draw = gfx.create_draw();
        draw.image(self.pixels.texture()).scale(1.0, 1.0);
        // Render map
        let rect_w = (width as f32 / (MAP_W*2) as f32);
        let rect_h = (height as f32 / MAP_H as f32);
        for j in 0..MAP_H {
            // draw the map
            for i in 0..MAP_W {
                if MAP.as_bytes()[i + j * MAP_W] as char == ' ' {
                    continue; // skip empty spaces
                }
                let rect_x = (i as f32) * rect_w;
                let rect_y = (j as f32) * rect_h;
                draw.rect((rect_x, rect_y), (rect_w, rect_h));
            }
        }

        // init player and draw them on canvas
        // Coord
        let player_x = 3.456; // player x position
        let player_y = 2.345;
        // Direction
        let player_a: f32 = 1.523; // player view direction
        const FOV: f32 = PI / 3.;
        draw.rect((player_x * rect_w, player_y * rect_h), (5., 5.));

        // DRAW FOV RAYCAST
        for i in 0..width {
            let mut t = 0.;
            // draw the visibility cone
            let angle = player_a - FOV / 2. + FOV * i as f32 / width as f32;
            let mut m = 0.;
            let mut n = 0.;
            while t < 20. {
                t = t + 0.05;
                let cx = player_x + t * angle.cos();
                let cy = player_y + t * angle.sin();
                if (MAP.as_bytes()[(cx as usize + cy as usize * MAP_W) as usize] as char != ' ') {
                    break;
                };

                let pix_x = cx * rect_w;
                let pix_y = cy * rect_h;

                m = pix_x;
                n = pix_y;
            }
            draw.path()
                .move_to(player_x * rect_w, player_y * rect_h)
                .line_to(m, n)
                .color(Color::WHITE)
                .stroke(0.05);
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
                .show(ctx, |ui| {
                    self.debug_ui(ui)
                });
        });

        gfx.render(&out);
    }
}

impl Game {
    fn debug_ui(&mut self, ui: &mut Ui) {
        Grid::new("debug_grid_1").show(ui, |ui| {
            let p = self.world.query_one_mut::<&mut Player>(self.player).unwrap();
            let (width, height) = self.pixels.dimensions();

            ui.label("X");
            Slider::new(&mut p.x, 0.0..=width as f32-1.0).ui(ui);
            ui.end_row();

            ui.label("Y");
            Slider::new(&mut p.y, 0.0..=height as f32-1.0).ui(ui);
            ui.end_row();
        });
    }
}
