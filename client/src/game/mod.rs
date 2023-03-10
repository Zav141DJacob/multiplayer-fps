mod controls;
pub(crate) mod ecs;
mod gameui;
mod minimap;
pub(crate) mod net;
mod raycast;
mod texture;
mod input;

use crate::game::minimap::Minimap;
use crate::game::raycast::*;
use crate::program::state::ProgramState;
use admin_client::program::Program;
use notan::app::{App, Color, Graphics, Plugins};

use anyhow::Context;
use notan::draw::CreateDraw;
use notan::prelude::*;
use std::fmt::{Display, Formatter};

use notan::egui::{EguiPluginSugar, Grid, Ui, Window};

use crate::game::ecs::ClientEcs;
use crate::game::input::InputHandler;
use crate::game::net::Connection;
use crate::game::raycast::sprites::Sprite;
use crate::game::texture::pixels::Pixels;
use crate::game::texture::ATLAS_MONSTER;
use common::ecs::components::{Player, Position, WeaponCrate};
use common::map::Map;
use common::{FromClientMessage, FromServerMessage};
use fps_counter::FPSCounter;
use glam::Vec2;
use hecs::Entity;
use itertools::Itertools;

use self::gameui::{GameUI, GameUiState};
use self::texture::WEAPON_CRATE;

const CAMERA_SENSITIVITY: f32 = 0.08; // rad
const FOV: f32 = 70.0;
const CEILING_COLOR: [u8; 4] = [100, 100, 170, 255];
const FLOOR_COLOR: [u8; 4] = [60, 120, 60, 255];

pub struct Game {
    ecs: ClientEcs,
    connection: Connection,
    my_entity: Entity,
    input: InputHandler,

    pixels: Pixels,
    minimap: Minimap,
    ray_caster: RayCaster,

    fps: FPSCounter,

    ui: GameUI,
    profiler: bool,
}

impl Game {
    pub fn new(
        app: &mut App, gfx: &mut Graphics, ecs: ClientEcs, connection: Connection, my_entity: Entity,
    ) -> Self {
        let (width, height) = gfx.size();
        let (width, height) = (width as usize, height as usize);

        let pixels = Pixels::new(width, height, gfx);
        let mut minimap = Minimap::new(ecs.resources.get::<Map>().unwrap().clone(), gfx);
        minimap.set_floor_color(FLOOR_COLOR.into());
        minimap.render_map(gfx);

        let fps = FPSCounter::new();

        let ray_caster = RayCaster::new(width, height, FOV);

        let ui_game_state = GameUiState {
            player_hp_max: 100,
            player_hp: 100,
            weapon_name: "SCAR".to_string(),
            max_ammo: 25,
            ammo: 15,
        };

        let ui = GameUI::new(ui_game_state, gfx);

        let input = InputHandler::new(app);

        Game {
            ecs,
            connection,
            my_entity,
            input,

            pixels,
            minimap,

            ray_caster,
            fps,
            ui,
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
    fn update(
        &mut self,
        app: &mut App,
        _assets: &mut Assets,
        _plugins: &mut Plugins,
    ) -> anyhow::Result<()> {
        self.accept_messages()?;

        self.input.tick(app);
        if let Some(state) = self.input.take_state() {
            self.connection.send(FromClientMessage::UpdateInputs(state))?;
        }

        self.ecs.tick(app.system_timer.delta_f32());

        Ok(())
    }

    fn draw(
        &mut self,
        app: &mut App,
        _assets: &mut Assets,
        gfx: &mut Graphics,
        plugins: &mut Plugins,
    ) -> anyhow::Result<()> {
        if self.input.mouse_locked() {
            app.window().set_cursor(CursorIcon::None);
        }

        let perspective = self.ray_caster.perspective(self.input.up_down_angle(), 0.6, 0.0);
        let horizon = (0.5 * self.pixels.height() as f32 + perspective.y_offset) as usize;

        let my_pos = self.ecs.world.query_one_mut::<&Position>(self.my_entity)
            .context("Couldn't query for own player entity")?;
        let my_pos = my_pos.0;
        let my_dir = Vec2::from_angle(self.input.peek_state().look_angle);

        self.pixels.clear_with_column(|y| {
            if y <= horizon {
                CEILING_COLOR
            } else {
                FLOOR_COLOR
            }
        });

        // Draw canvas background
        let (width, height) = self.pixels.dimensions();

        self.ray_caster.draw_walls(
            &mut self.pixels,
            my_pos,
            my_dir,
            perspective,
            &*self.ecs.resources.get::<Map>()?,
        );

        let mut enemy_sprites = self
            .ecs
            .world
            .query_mut::<&Position>()
            .with::<&Player>()
            .into_iter()
            .filter(|(entity, _)| self.my_entity != *entity)
            .map(|(_, pos)| pos.0)
            .map(|pos| Sprite::new(&ATLAS_MONSTER[0], pos, Vec2::ONE, 0.0))
            .collect_vec();

        let mut crate_sprites = self
            .ecs
            .world
            .query_mut::<&Position>()
            .with::<&WeaponCrate>()
            .into_iter()
            .map(|(_, pos)| pos.0)
            .map(|pos| Sprite::new(&WEAPON_CRATE, pos, Vec2::new(0.5, 0.5), 0.0))
            .collect_vec();

        self.ray_caster
            .draw_sprites(&mut self.pixels, my_pos, my_dir, perspective, &mut crate_sprites);

        self.ray_caster
            .draw_sprites(&mut self.pixels, my_pos, my_dir, perspective, &mut enemy_sprites);

        let mut draw = gfx.create_draw();

        // Render pixels
        self.pixels.flush(gfx);
        self.pixels.draw(&mut draw);

        // Draw UI
        self.ui.draw_health(&mut draw, width, height);
        self.ui.draw_weapon_stats(&mut draw, width, height);

        // Drawing minimap
        self.minimap.draw(&mut draw, width, height);

        self.minimap.render_vision(
            &mut draw,
            width,
            height,
            my_pos,
            Color::new(0.2, 0.2, 0.2, 1.0),
            self.ray_caster.minimap_rays(),
        );

        self.minimap
            .render_player_location(&mut draw, width, height, my_pos, Color::RED);

        // Draw enemies on map
        for sprite in enemy_sprites.iter() {
            self.minimap.render_entity_location(
                &mut draw,
                width,
                height,
                sprite.position,
                sprite.texture.dominant().into(),
            );
        }

        // Draw weapon crates on map
        for sprite in crate_sprites.iter() {
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
                if let Ok(mut server_ui) = self.ecs.resources.get_mut::<Program>() {
                    Window::new("Server")
                        .collapsible(true)
                        .resizable(true)
                        .show(ctx, |ui| server_ui.draw(ui, app));
                }

                Window::new("Client debug")
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

        Ok(())
    }

    fn event(&mut self, _app: &mut App, _assets: &mut Assets, _plugins: &mut Plugins, event: Event) -> anyhow::Result<()> {
        self.input.handle_event(event);
        Ok(())
    }
}

impl Game {
    fn accept_messages(&mut self) -> anyhow::Result<()> {
        while let Some(message) = self.connection.receive()? {
            if let FromServerMessage::EcsChanges(changes) = message {
                for change in changes {
                    self.ecs.handle_protocol(change)?;
                }
            }
        }

        Ok(())
    }

    fn debug_ui(&mut self, ui: &mut Ui) {
        if ui.checkbox(&mut self.profiler, "Profiler").changed() {
            puffin::set_scopes_on(self.profiler);
        };

        Grid::new("debug_grid_1").show(ui, |_ui| {});
    }
}
