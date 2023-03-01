use std::{
    fmt::{Display, Formatter},
    net::{AddrParseError, IpAddr},
    num::ParseIntError,
};

use common::defaults::{IP, PORT};
use notan::{
    egui::{self, EguiPluginSugar, Id},
    prelude::{App, Assets, Color, Graphics, Plugins},
};

use crate::{connecting::Connecting, error::ErrorState, program::state::ProgramState};

use super::Menu;

enum NextState {
    Menu,
    Game,
}

#[derive(Clone)]
enum InputErrors {
    Port(ParseIntError),
    Ip(AddrParseError),
}

impl ToString for InputErrors {
    fn to_string(&self) -> String {
        match self {
            InputErrors::Port(e) => e.to_string(),
            InputErrors::Ip(e) => e.to_string(),
        }
    }
}

#[derive(Clone)]
struct ErrorWindow {
    error: InputErrors,
    id: u16,
    is_open: bool,
}

impl ErrorWindow {
    pub fn new(error: InputErrors, id: u16) -> ErrorWindow {
        ErrorWindow {
            error,
            id,
            is_open: true,
        }
    }
}

#[derive(Default)]
pub struct ServerSelectionMenu {
    errors: Vec<ErrorWindow>,
    next_state: Option<NextState>,
    ip: String,
    port: String,

    processed_ip: Option<IpAddr>,
    processed_port: Option<u16>,
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
            ip: IP.to_string(),
            port: PORT.to_string(),

            processed_ip: None,
            processed_port: None,
            errors: Vec::new(),
        }
    }

    fn process_inputs(&mut self) -> bool {
        let new_id = || match self.errors.last() {
            Some(error) => error.id + 1,
            None => 0,
        };

        self.processed_port = match self.port.parse() {
            Ok(p) => Some(p),
            Err(error) => {
                self.errors
                    .push(ErrorWindow::new(InputErrors::Port(error), new_id()));
                return false;
            }
        };

        self.processed_ip = match self.ip.parse() {
            Ok(ip) => Some(ip),
            Err(error) => {
                self.errors
                    .push(ErrorWindow::new(InputErrors::Ip(error), new_id()));
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
                for error in self.errors.iter_mut() {
                    let title = match error.error {
                        InputErrors::Port(_) => "Error: Invalid port",
                        InputErrors::Ip(_) => "Error: Invalid IP",
                    };

                    egui::Window::new(title)
                        .id(Id::new(error.id))
                        .open(&mut error.is_open)
                        .show(ctx, |ui| ui.label(error.error.to_string()));
                }

                self.errors = self
                    .errors
                    .clone()
                    .into_iter()
                    .filter(|error| error.is_open)
                    .collect();

                ui.vertical_centered(|ui| {
                    ui.heading("Join Server");
                    ui.add_space(10.0);

                    ui.label("IP & Port:");
                    ui.text_edit_singleline(&mut self.ip);
                    ui.text_edit_singleline(&mut self.port);

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
                let state =
                    Connecting::new(self.processed_ip.unwrap(), self.processed_port.unwrap())
                        .map(|v| v.into())
                        .unwrap_or_else(|err| ErrorState::from(&*err).into());
                Some(state)
            }
            NextState::Menu => Some(Menu::new().into()),
        }
    }
}
