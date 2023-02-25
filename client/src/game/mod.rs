mod controls;
mod ecs;
mod minimap;
mod raycast;
mod texture;

use crate::game::controls::*;
use crate::game::minimap::Minimap;
use crate::game::raycast::*;
use crate::program::state::ProgramState;
use common::map::Map;
use hecs::Entity;
use notan::app::{App, Color, Graphics, Plugins};

use notan::draw::{CreateDraw, DrawImages};
use notan::prelude::*;
use std::fmt::{Display, Formatter};


use notan::egui::{EguiPluginSugar, Grid, Slider, Ui, Widget, Window};
// use notan::prelude::{Assets, Texture, KeyCode};

use fps_counter::FPSCounter;
use glam::f32::Vec2;
use crate::game::raycast::sprites::{default_sprites, Sprite};
use crate::game::texture::draw_column::Perspective;
use crate::game::texture::pixels::Pixels;

const PLAYER_SPEED: f32 = 0.1;
const CAMERA_SENSITIVITY: f32 = 0.08; // rad

pub struct Game {
    world: hecs::World,
    map: Map,

    pixels: Pixels,
    minimap: Minimap,
    ray_caster: RayCaster,

    player: Entity,
    sprites: Vec<Sprite>,

    fps: FPSCounter,

    profiler: bool,
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

        let sprites = default_sprites();

        let fps = FPSCounter::new();

        let ray_caster = RayCaster::new(width, height);

        Game {
            world,
            map,
            player,
            pixels,
            minimap,
            ray_caster,
            sprites,
            fps,
            profiler: false,
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Game")
    }
}

impl ProgramState for Game {
    fn update(&mut self, app: &mut App, _assets: &mut Assets, _plugins: &mut Plugins) {
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
        _app: &mut App,
        _assets: &mut Assets,
        gfx: &mut Graphics,
        plugins: &mut Plugins,
    ) {
        self.pixels.clear(Color::BLACK);

        let p = self
            .world
            .query_one_mut::<(&Position, &Direction)>(self.player)
            .unwrap();

        // Draw canvas background
        let (width, height) = self.pixels.dimensions();

        let up_down_angle = -10.0_f32.to_radians();
        let perspective = Perspective::new(up_down_angle, 0.6, 0.0);

        self.ray_caster.draw_walls(
            &mut self.pixels,
            p.0.xy,
            p.1.xy,
            perspective,
            &self.map,
        );

        self.ray_caster.draw_sprites(
            &mut self.pixels,
            p.0.xy,
            p.1.xy,
            perspective,
            &mut self.sprites,
        );

        let mut draw = gfx.create_draw();

        // Render pixels
        self.pixels.flush(gfx);
        draw.image(self.pixels.texture());

        // Drawing minimap
        self.minimap.draw(&mut draw, width, height);

        self.minimap.render_vision(
            &mut draw,
            width,
            height,
            p.0.xy,
            Color::new(0.2, 0.2, 0.2, 1.0),
            self.ray_caster.minimap_rays(),
        );

        self.minimap
            .render_player_location(&mut draw, width, height, p.0.xy, Color::RED);

        // Draw enemies on map
        for sprite in self.sprites.iter() {
            self.minimap.render_entity_location(
                &mut draw,
                width,
                height,
                sprite.position,
                sprite.texture.dominant().into(),
            );
        }

        gfx.render(&draw);



        // Render egui
        {
            puffin::profile_scope!("render egui");

            let out = plugins.egui(|ctx| {
                Window::new("Debug")
                    .collapsible(true)
                    .resizable(false)
                    .show(ctx, |ui| self.debug_ui(ui));

                if self.profiler {
                    puffin_egui::profiler_window(ctx);
                }

                notan::egui::Area::new("fps-counter")
                    .fixed_pos(notan::egui::pos2(0.0, 0.0))
                    .show(ctx, |ui| {
                        ui.label(format!("FPS: {}", self.fps.tick()));
                    });
            });

            gfx.render(&out);
        }


    }
}

impl Game {
    fn debug_ui(&mut self, ui: &mut Ui) {
        if ui.checkbox(&mut self.profiler, "Profiler").changed() {
            puffin::set_scopes_on(self.profiler);
        };

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
