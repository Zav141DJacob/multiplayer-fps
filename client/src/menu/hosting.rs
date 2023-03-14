use std::{
    fmt::{Display, Formatter},
    net::UdpSocket,
};

use admin_client::program::Program;
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
struct ErrorWindow {
    error: String,
    id: u16,
    is_open: bool,
}

impl ErrorWindow {
    pub fn new(error: String, id: u16) -> ErrorWindow {
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

    server: Option<Program>,
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

            server: None,
        }
    }

    fn add_error(&mut self, error: String) {
        let id = match self.errors.last() {
            Some(error) => error.id + 1,
            None => 0,
        };

        self.errors.push(ErrorWindow::new(error, id));
    }

    fn process_inputs(&mut self) -> bool {
        self.processed_port = match self.port.parse() {
            Ok(p) => Some(p),
            Err(error) => {
                self.add_error(error.to_string());
                return false;
            }
        };

        if !udp_port_is_available(self.processed_port.unwrap()) {
            self.add_error("Port is already used".to_string());
            return false;
        }

        let mut p = Program::new(IP, self.processed_port.unwrap(), false);
        if let Err(error) = p.run() {
            self.add_error(error.to_string());
            return false;
        }

        self.server = Some(p);
        self.next_state = Some(NextState::Game);
        true
    }
}

fn udp_port_is_available(port: u16) -> bool {
    UdpSocket::bind((IP, port)).is_ok()
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
                        self.process_inputs();
                    }

                    ui.add_space(10.0);
                    ui.vertical_centered(|ui| {
                        ui.set_width(ui.available_width() / 4.0);
                        ui.horizontal(|ui| {
                            if ui.button("Host").clicked() {
                                self.process_inputs();
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
                let state = Connecting::new(IP, self.processed_port.unwrap(), self.server.clone())
                    .map(|v| v.into())
                    .unwrap_or_else(|err| ErrorState::from(&*err).into());
                Some(state)
            }
            NextState::Menu => Some(Menu::new().into()),
        }
    }
}
