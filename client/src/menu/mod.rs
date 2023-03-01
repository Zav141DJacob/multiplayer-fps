use std::fmt::{Display, Formatter};

use notan::app::{App, Graphics, Plugins};
use notan::egui::{self, EguiPluginSugar};
use notan::prelude::{Assets, Color};

use common::defaults::{IP, PORT};

use crate::connecting::Connecting;
use crate::error::ErrorState;
use crate::net_test::NetworkTest;
use crate::program::state::ProgramState;

#[derive(Default)]
pub struct Menu {
    next_state: Option<NextState>,
}

enum NextState {
    Game,
    NetworkTest,
}

impl Display for Menu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Menu")
    }
}

impl ProgramState for Menu {
    fn draw(
        &mut self,
        app: &mut App,
        _assets: &mut Assets,
        gfx: &mut Graphics,
        plugins: &mut Plugins,
    ) -> anyhow::Result<()> {
        let mut output = plugins.egui(|ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Multiplayer FPS");
                    ui.add_space(10.0);

                    if ui.button("Start").clicked() {
                        self.next_state = Some(NextState::Game)
                    }

                    if ui.button("Network Test").clicked() {
                        self.next_state = Some(NextState::NetworkTest)
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

        Ok(())
    }

    fn change_state(
        &mut self,
        _app: &mut App,
        _assets: &mut Assets,
        _gfx: &mut Graphics,
        _plugins: &mut Plugins,
    ) -> Option<Box<dyn ProgramState>> {
        match self.next_state.take()? {
            NextState::Game => {
                let state = Connecting::new(IP, PORT)
                    .map(|v| v.into())
                    .unwrap_or_else(|err| {
                        ErrorState::from(&*err).into()
                    });
                Some(state)
            }
            NextState::NetworkTest => Some(NetworkTest::new().into()),
        }
    }
}
