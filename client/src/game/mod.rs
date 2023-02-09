mod pixels;
mod minimap;

use std::fmt::{Display, Formatter};
use hecs::Entity;
use notan::app::{App, Color, Graphics, Plugins};
use notan::draw::{CreateDraw, DrawImages, DrawTransform};
use notan::egui::{DragValue, EguiPluginSugar, Grid, Slider, Ui, Widget, Window};
use notan::prelude::{Assets, Texture, KeyCode};
use common::map::Map;
use crate::game::pixels::Pixels;
use crate::game::minimap::Minimap;
use crate::program::state::ProgramState;

pub struct Game {
    world: hecs::World,
    map: Map,

    pixels: Pixels,
    minimap: Minimap,

    foo: usize,
    player: Entity,
}

#[derive(Debug)]
pub struct Player {
    x: f32,
    y: f32,
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
        let player = world.spawn((Player{ x: 1.5, y: 1.5 },));

        Game {
            world,
            map,
            foo: 0,
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

        if app.keyboard.is_down(KeyCode::W) && p.y > 0.0 {
            p.y -= 0.2;
        }

        if app.keyboard.is_down(KeyCode::A) && p.x > 0.0 {
            p.x -= 0.2;
        }

        if app.keyboard.is_down(KeyCode::S) && p.y < (self.pixels.dimensions().1 - 1) as f32 {
            p.y += 0.2;
        }

        if app.keyboard.is_down(KeyCode::D) && p.x < (self.pixels.dimensions().0 - 1) as f32 {
            p.x += 0.2;
        }
    }

    fn draw(&mut self, app: &mut App, assets: &mut Assets, gfx: &mut Graphics, plugins: &mut Plugins) {
        let p = self.world.query_one_mut::<&mut Player>(self.player).unwrap();

        // Draw a red dot
        let (width, height) = self.pixels.dimensions();
        self.foo %= width * height;
        let x = p.x;
        let y = p.y;
        self.pixels.set_color(x as usize, y as usize, Color::RED);

        // Render pixels
        self.pixels.flush(gfx);
        self.pixels.clear(Color::BLACK);

        let mut draw = gfx.create_draw();
        draw.image(self.pixels.texture()).scale(1.0, 1.0);


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