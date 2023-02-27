use std::fmt::{Display, Formatter};

use common::defaults::PORT;
use notan::app::{App, Graphics, Plugins};
use notan::egui::{self, EguiPluginSugar};
use notan::prelude::{Assets, Color};

use crate::game::Game;
use crate::net_test::NetworkTest;
use crate::program::state::ProgramState;

pub mod hosting;
pub mod server_selection;

#[derive(Default)]
pub struct Menu {
    next_state: Option<NextState>,

    menu_state: SubMenu,
    port: String,
    ip: String,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            next_state: None,

            menu_state: SubMenu::default(),
            port: PORT.to_string(),
            ip: String::new(),
        }
    }
}

enum NextState {
    Game,
    NetworkTest,
}

#[derive(Default)]
enum SubMenu {
    #[default]
    Menu,
    ServerSelection,
    HostingMenu,
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
    ) {
        let mut output = plugins.egui(|ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                match self.menu_state {
                    SubMenu::Menu => {
                        ui.vertical_centered(|ui| {
                            ui.heading("Multiplayer FPS");
                            ui.add_space(10.0);

                            // TODO: Center properly
                            ui.horizontal(|ui| {
                                if ui.button("Quick Play").clicked() {
                                    self.next_state = Some(NextState::Game)
                                }

                                if ui.button("Join Server").clicked() {
                                    self.menu_state = SubMenu::ServerSelection;
                                }
                            });

                            if ui.button("Host Server").clicked() {
                                self.menu_state = SubMenu::HostingMenu;
                            }

                            if ui.button("Network Test").clicked() {
                                self.next_state = Some(NextState::NetworkTest)
                            }

                            if ui.button("Quit").clicked() {
                                app.exit()
                            }
                        });
                    }
                    SubMenu::ServerSelection => server_selection::execute(self, ui),
                    SubMenu::HostingMenu => hosting::execute(self, ui),
                }
            });
        });

        output.clear_color(Color::BLACK);

        if output.needs_repaint() {
            gfx.render(&output);
        }
    }

    fn change_state(
        &mut self,
        _app: &mut App,
        _assets: &mut Assets,
        gfx: &mut Graphics,
        _plugins: &mut Plugins,
    ) -> Option<Box<dyn ProgramState>> {
        match self.next_state.take()? {
            NextState::Game => Some(Game::new(gfx).into()),
            NextState::NetworkTest => Some(NetworkTest::new().into()),
        }
    }
}
