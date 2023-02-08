use std::fmt::{Display, Formatter};
use notan::app::{App, Graphics, Plugins};
use notan::egui::{self, Align2, Area, EguiPluginSugar, Vec2};
use notan::prelude::{Assets, Color};
use tracing::error;
use crate::game::Game;
use crate::program::state::ProgramState;

#[derive(Default)]
pub struct Menu {
    next_state: Option<Box<dyn ProgramState>>
}

impl Display for Menu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Menu")
    }
}

impl ProgramState for Menu {
    fn draw(&mut self, app: &mut App, assets: &mut Assets, gfx: &mut Graphics, plugins: &mut Plugins) {
        let mut output = plugins.egui(|ctx| {

            egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Multiplayer FPS");
                        ui.add_space(10.0);

                        if ui.button("Start").clicked() {
                            self.next_state = Some(start_game(gfx).into())
                        }

                        if ui.button("Quit").clicked() {
                            app.exit()
                        }
                    });
            });
        });

        output.clear_color(Color::BLACK);

        if output.needs_repaint() {
            gfx.render(&output);
        }
    }

    fn change_state(&mut self, app: &mut App, assets: &mut Assets, gfx: &mut Graphics, plugins: &mut Plugins) -> Option<Box<dyn ProgramState>> {
        self.next_state.take()
    }
}

fn start_game(gfx: &mut Graphics) -> Game {
    Game::new(gfx)
}