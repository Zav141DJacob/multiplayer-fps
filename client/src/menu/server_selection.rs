use std::{
    fmt::{Display, Formatter},
    net::{SocketAddr, ToSocketAddrs},
};

use common::defaults::{IP, PORT};
use notan::{
    egui::{self, EguiPluginSugar},
    prelude::{App, Assets, Color, Graphics, Plugins},
};

use crate::{
    connecting::Connecting, error::ErrorState, errorwindow::ErrorWindows,
    program::state::ProgramState,
};

use super::Menu;

enum NextState {
    Menu,
    Game,
}

#[derive(Default)]
pub struct ServerSelectionMenu {
    errors: ErrorWindows,
    next_state: Option<NextState>,
    ip: String,
    username: String,

    processed_ip: Option<SocketAddr>,
}

impl Display for ServerSelectionMenu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ServerSelectionMenu")
    }
}

impl ServerSelectionMenu {
    pub fn new() -> ServerSelectionMenu {
        ServerSelectionMenu {
            next_state: None,
            ip: format!("{IP}:{PORT}"),
            username: String::new(),

            processed_ip: None,
            errors: ErrorWindows::new(),
        }
    }

    fn process_inputs(&mut self) -> bool {
        self.processed_ip = match self.ip.to_socket_addrs() {
            Ok(mut ip) => Some(ip.next().unwrap()),
            Err(error) => {
                self.errors.add_error(error.to_string());
                return false;
            }
        };

        true
    }
}

impl ProgramState for ServerSelectionMenu {
    fn draw(
        &mut self,
        _app: &mut App,
        _assets: &mut Assets,
        gfx: &mut Graphics,
        plugins: &mut Plugins,
    ) -> anyhow::Result<()> {
        let mut output = plugins.egui(|ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                self.errors.draw_errors(ctx);

                ui.vertical_centered(|ui| {
                    ui.heading("Join Server");
                    ui.add_space(10.0);

                    ui.label("IP & Port:");
                    ui.text_edit_singleline(&mut self.ip);

                    ui.label("Username");
                    ui.text_edit_singleline(&mut self.username);

                    ui.add_space(10.0);
                    ui.vertical_centered(|ui| {
                        ui.set_width(ui.available_width() / 4.0);
                        ui.horizontal(|ui| {
                            if ui.button("Join").clicked() {
                                if !self.process_inputs() {
                                    return;
                                }

                                self.next_state = Some(NextState::Game);
                            }
                            if ui.button("Back").clicked() {
                                self.next_state = Some(NextState::Menu);
                            }
                        })
                    });
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
                let processed = self.processed_ip.unwrap();
                let state = Connecting::new(processed.ip(), processed.port(), None, &self.username)
                    .map(|v| v.into())
                    .unwrap_or_else(|err| ErrorState::from(&*err).into());
                Some(state)
            }
            NextState::Menu => Some(Menu::new().into()),
        }
    }
}
