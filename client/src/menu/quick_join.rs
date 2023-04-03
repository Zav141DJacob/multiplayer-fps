use std::{
    fmt::{Display, Formatter},
    net::ToSocketAddrs,
};

use common::defaults::{DEFAULT_PLAYER_NAME, PORT, QUICK_JOIN_IP};
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
pub struct QuickJoinMenu {
    errors: ErrorWindows,
    username: String,
    next_state: Option<NextState>,
}

impl Display for QuickJoinMenu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "QuickJoinMenu")
    }
}

impl QuickJoinMenu {
    pub fn new() -> QuickJoinMenu {
        QuickJoinMenu {
            errors: ErrorWindows::new(),
            username: DEFAULT_PLAYER_NAME.to_string(),
            next_state: None,
        }
    }

    fn process_inputs(&mut self) -> bool {
        match format!("{QUICK_JOIN_IP}:{PORT}").to_socket_addrs() {
            Ok(mut s) => match s.next() {
                Some(_) => true,
                None => {
                    self.errors
                        .add_error(String::from("Failed to find domain IP"));
                    false
                }
            },
            Err(e) => {
                self.errors.add_error(e.to_string());
                false
            }
        }
    }
}

impl ProgramState for QuickJoinMenu {
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
                    ui.heading("Quick Join Server");
                    ui.add_space(10.0);

                    ui.label(format!("Joining server: {QUICK_JOIN_IP}:{PORT}"));
                    ui.add_space(5.0);

                    ui.label("Username");
                    ui.text_edit_singleline(&mut self.username);

                    ui.add_space(10.0);
                    ui.vertical_centered(|ui| {
                        ui.set_width(ui.available_width() / 4.0);
                        ui.horizontal(|ui| {
                            if ui.button("Join").clicked() && self.process_inputs() {
                                self.next_state = Some(NextState::Game)
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
                let socketaddr = format!("{QUICK_JOIN_IP}:{PORT}")
                    .to_socket_addrs()
                    .unwrap()
                    .next()
                    .unwrap();

                let state =
                    Connecting::new(socketaddr.ip(), socketaddr.port(), None, &self.username)
                        .map(|v| v.into())
                        .unwrap_or_else(|err| ErrorState::from(&*err).into());
                Some(state)
            }
            NextState::Menu => Some(Menu::new().into()),
        }
    }
}
