use std::{
    fmt::{Display, Formatter},
    num::ParseIntError, net::{IpAddr, Ipv4Addr},
};

use common::defaults::{IP, PORT};
use notan::{
    egui::{self, EguiPluginSugar, Id},
    prelude::{App, Assets, Color, Graphics, Plugins},
};

use crate::{connecting::Connecting, error::ErrorState, program::state::ProgramState};

use super::Menu;

pub fn host_server(port: &str) {
    // TODO: complete server hosting part
    // cargo run --release --bin server --port 1337
    // server::run_server("127.0.0.1".parse().unwrap(), port.parse().unwrap()).unwrap()
    todo!()
}

enum NextState {
    Menu,
    Game,
}

#[derive(Clone)]
struct ErrorWindow {
    error: ParseIntError,
    id: u16,
    is_open: bool,
}

impl ErrorWindow {
    pub fn new(error: ParseIntError, id: u16) -> ErrorWindow {
        ErrorWindow {
            error,
            id,
            is_open: true,
        }
    }
}
#[derive(Default)]
pub struct HostingMenu {
    errors: Vec<ErrorWindow>,
    next_state: Option<NextState>,
    port: String,

    processed_port: Option<u16>,
}

impl Display for HostingMenu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "HostingMenu")
    }
}

impl HostingMenu {
    pub fn new() -> HostingMenu {
        HostingMenu {
            errors: Vec::new(),
            next_state: None,
            port: PORT.to_string(),
            processed_port: None,
        }
    }

    fn process_inputs(&mut self) -> bool {
        self.processed_port = match self.port.parse() {
            Ok(p) => Some(p),
            Err(error) => {
                let id = match self.errors.last() {
                    Some(error) => error.id + 1,
                    None => 0,
                };

                self.errors.push(ErrorWindow::new(error, id));
                return false;
            }
        };

        true
    }
}

impl ProgramState for HostingMenu {
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
                    egui::Window::new("Error: Invalid port")
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
                    ui.heading("Host Server");
                    ui.add_space(10.0);

                    ui.label("Port:");

                    let response = ui.add(
                        egui::TextEdit::singleline(&mut self.port).hint_text(PORT.to_string()),
                    );

                    // When you press enter it submits
                    if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                        if !self.process_inputs() {
                            return;
                        }

                        host_server(&self.port);
                        self.next_state = Some(NextState::Game);
                    }

                    ui.add_space(10.0);
                    ui.vertical_centered(|ui| {
                        ui.set_width(ui.available_width() / 4.0);
                        ui.horizontal(|ui| {
                            if ui.button("Host").clicked() {
                                if !self.process_inputs() {
                                    return;
                                }

                                host_server(&self.port);
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
                let state = Connecting::new(
                    IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                    self.processed_port.unwrap(),
                )
                .map(|v| v.into())
                .unwrap_or_else(|err| ErrorState::from(&*err).into());
                Some(state)
            }
            NextState::Menu => Some(Menu::new().into()),
        }
    }
}
