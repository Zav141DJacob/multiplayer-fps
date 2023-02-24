mod controls;
mod ecs;
mod enemy;
mod minimap;
mod pixels;
mod raycast;
mod textures;

use crate::game::controls::*;
use crate::game::enemy::*;
use crate::game::minimap::Minimap;
use crate::game::pixels::Pixels;
use crate::game::raycast::*;
use crate::game::textures::Texture;
use crate::program::state::ProgramState;
use common::map::{Map, Wall};
use hecs::Entity;
use notan::app::{App, Color, Graphics, Plugins};

use notan::draw::{CreateDraw, DrawImages, DrawTransform};
use notan::prelude::*;
use std::f32::consts::PI;
use std::fmt::{Display, Formatter};

use notan::egui::{DragValue, EguiPluginSugar, Grid, Slider, Ui, Widget, Window};
// use notan::prelude::{Assets, Texture, KeyCode};

use fps_counter::FPSCounter;
use glam::f32::Vec2;
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

        let enemies = add_enemies();

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
        let p = self
            .world
            .query_one_mut::<(&mut Position, &mut Direction)>(self.player)
            .unwrap();
        let w = self.map.get_width() as f32;
        let h = self.map.get_height() as f32;
        handle_keyboard_input(app, w, h, p);
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

        let mut draw = gfx.create_draw();
        draw.image(self.pixels.texture()).scale(1.0, 1.0);

        let mut minimap_rays = Vec::new();

        // Vector (distance from player to walls)
        let mut depth_map = vec![0.; width];
        draw_walls(
            &mut self.pixels,
            width,
            height,
            p,
            self.map.clone(),
            &self.texture,
            &mut minimap_rays,
            &mut depth_map,
        );
        draw_enemies(
            &mut self.pixels,
            width,
            height,
            p,
            &self.enemies,
            &self.enemy_texture,
            &mut depth_map,
        );
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

        // Draw enemies on map
        for enemy in self.enemies.iter() {
            self.minimap.render_entity_location(
                &mut draw,
                width,
                height,
                enemy.position.xy,
                Color::WHITE,
            );
        }

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
                .show(ctx, |ui| {
                    ui.label("FPS: ".to_owned() + &self.fps.tick().to_string())
                });
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
