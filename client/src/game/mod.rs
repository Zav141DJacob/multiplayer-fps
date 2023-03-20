pub(crate) mod ecs;
mod gameui;
mod input;
mod minimap;
pub(crate) mod net;
mod raycast;
mod texture;

use crate::args::ARGS;
use crate::game::minimap::Minimap;
use crate::game::raycast::*;
use crate::program::state::ProgramState;
use admin_client::program::Program;
use common::defaults::{MINIMAP_SCALE, PLAYER_MAX_HP};
use notan::app::{App, Color, Graphics, Plugins};

use anyhow::Context;
use notan::draw::CreateDraw;
use notan::prelude::*;
use std::fmt::{Display, Formatter};

use notan::egui::{Color32, EguiPluginSugar, Frame, Grid, Pos2, Ui, Window};

use crate::game::ecs::component::{Height, RenderSprite, Scale};
use crate::game::ecs::{ClientEcs, MyEntity};
use crate::game::input::InputHandler;
use crate::game::net::Connection;
use crate::game::raycast::sprites::Sprite;
use crate::game::texture::pixels::Pixels;
use common::ecs::components::{Deaths, Health, HeldWeapon, Kills, Player, Position};
use common::map::Map;
use common::{FromClientMessage, FromServerMessage};
use fps_counter::FPSCounter;
use glam::Vec2;
use hecs::Entity;
use itertools::Itertools;

use self::gameui::{GameUI, GameUiState};

const CAMERA_SENSITIVITY: f32 = 0.08; // rad
const FOV: f32 = 70.0;
const CEILING_COLOR: [u8; 4] = [110, 110, 190, 255];
const FLOOR_COLOR: [u8; 4] = [90, 90, 90, 255];

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
        app: &mut App,
        gfx: &mut Graphics,
        mut ecs: ClientEcs,
        connection: Connection,
        my_entity: Entity,
    ) -> Self {
        let (width, height) = gfx.size();
        let (width, height) = (width as usize, height as usize);

        ecs.resources.insert(MyEntity(my_entity));

        let pixels = Pixels::new(width, height, gfx);
        let mut minimap = Minimap::new(ecs.resources.get::<Map>().unwrap().clone(), gfx);
        minimap.set_minimap_scale(Vec2::splat(MINIMAP_SCALE));
        minimap.set_floor_color(FLOOR_COLOR.into());
        minimap.render_map(gfx);

        let fps = FPSCounter::new();

        let ray_caster = RayCaster::new(width, height, FOV);

        let ui = GameUI::new(GameUiState::new(), gfx);

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
            self.connection
                .send(FromClientMessage::UpdateInputs(state))?;
        }

        let dt = app.system_timer.delta_f32();
        self.ecs.tick(dt);

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

        let perspective = self
            .ray_caster
            .perspective(self.input.up_down_angle(), 0.65, 0.0);
        let horizon = (0.5 * self.pixels.height() as f32 + perspective.y_offset) as usize;

        let my_pos = self
            .ecs
            .world
            .query_one_mut::<&Position>(self.my_entity)
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

        let mut sprites = self
            .ecs
            .world
            .query_mut::<(&Position, &RenderSprite, Option<&Scale>, Option<&Height>)>()
            .into_iter()
            .filter(|(entity, _)| self.my_entity != *entity)
            .map(|(_, (pos, sprite, scale, height))| {
                (
                    pos.0,
                    sprite.tex,
                    scale.map(|v| v.0).unwrap_or(Vec2::ONE),
                    height.map(|v| v.0).unwrap_or(0.0),
                )
            })
            .map(|(pos, tex, scale, height)| Sprite::new(tex, pos, scale, height))
            .collect_vec();

        self.ray_caster
            .draw_sprites(&mut self.pixels, my_pos, my_dir, perspective, &mut sprites);

        let mut draw = gfx.create_draw();

        // Render pixels
        self.pixels.flush(gfx);
        self.pixels.draw(&mut draw);

        // set UI game state
        let (health, weapon) = self
            .ecs
            .world
            .query_one_mut::<(&Health, &HeldWeapon)>(self.my_entity)
            .unwrap();
        self.ui.set_game_state(GameUiState {
            player_hp_max: PLAYER_MAX_HP,
            player_hp: health.0,
            weapon_name: weapon.gun.to_string(),
            max_ammo: weapon.gun.max_ammo(),
            ammo: weapon.ammo,
        });
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
            Color::new(1.0, 1.0, 1.0, 0.1),
            &self.ray_caster,
        );

        self.minimap
            .render_player_location(&mut draw, width, height, my_pos, Color::RED);

        // Draw sprites on map
        for sprite in sprites.iter() {
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

                if ARGS.debug {
                    Window::new("Client debug")
                        .collapsible(true)
                        .resizable(false)
                        .show(ctx, |ui| self.debug_ui(ui));
                }

                if self.profiler {
                    puffin_egui::profiler_window(ctx);
                }

                let leaderboard_frame = Frame {
                    fill: Color32::from_rgba_premultiplied(0, 0, 0, 150),
                    rounding: 5.0.into(),
                    inner_margin: 10.0.into(),
                    outer_margin: 0.5.into(), // so the stroke is within the bounds
                    ..Default::default()
                };

                if ctx.input().key_down(notan::egui::Key::Tab) {
                    Window::new("leaderboard")
                        .frame(leaderboard_frame)
                        .collapsible(false)
                        .resizable(false)
                        .fixed_pos(Pos2 { x: 550.0, y: 5.0 })
                        .show(ctx, |ui| {
                            for (_, (player, kills, deaths)) in
                                self.ecs.world.query::<(&Player, &Kills, &Deaths)>().iter()
                            {
                                let kd: f32 = if kills.0 == 0 || deaths.0 == 0 {
                                    0.0
                                } else {
                                    kills.0 as f32 / deaths.0 as f32
                                };

                                ui.label(format!(
                                    "[{}] K: {}, D: {}, K/D: {kd}",
                                    player.name, kills.0, deaths.0,
                                ));
                            }
                        });
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

    fn event(
        &mut self,
        _app: &mut App,
        _assets: &mut Assets,
        _plugins: &mut Plugins,
        event: Event,
    ) -> anyhow::Result<()> {
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
