mod pixels;
mod minimap;

use std::fmt::{Display, Formatter};
use notan::app::{App, Color, Graphics, Plugins};
use notan::draw::{CreateDraw, DrawImages, DrawTransform};
use notan::egui::{DragValue, EguiPluginSugar, Grid, Slider, Ui, Widget, Window, Vec2};
use notan::prelude::{Assets, Texture};
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
}

impl Game {
    pub fn new(gfx: &mut Graphics) -> Self {
        let (width, height) = gfx.size();
        let (width, height) = (width as usize, height as usize);
        
        let  map = Map::default();

        let pixels = Pixels::new(width, height, gfx);
        let mut minimap = Minimap::new(map.clone(), gfx);
        minimap.render_map(gfx);

        Game {
            world: hecs::World::new(),
            map,
            foo: 0,

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
        self.foo += 1;
    }

    fn draw(&mut self, app: &mut App, assets: &mut Assets, gfx: &mut Graphics, plugins: &mut Plugins) {
        // Draw a red dot
        let (width, height) = self.pixels.dimensions();
        self.foo %= width * height;
        let x = self.foo % width;
        let y = self.foo / width;
        self.pixels.set_color(x, y, Color::RED);

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
            let (width, height) = self.pixels.dimensions();
            let mut x = self.foo % width;
            let mut y = self.foo / width;

            ui.label("X");
            Slider::new(&mut x, 0..=width-1).ui(ui);
            ui.end_row();

            ui.label("Y");
            Slider::new(&mut y, 0..=height-1).ui(ui);
            ui.end_row();

            self.foo = (y * width + x);
        });
    }
}