use std::fmt::{Display, Formatter};

use notan::app::{App, Graphics, Plugins};
use notan::egui::{self, EguiPluginSugar, Ui};
use notan::prelude::{Assets, Color};

use crate::args::ARGS;
use crate::net_test::NetworkTest;
use crate::program::state::ProgramState;

use self::hosting::HostingMenu;
use self::quick_join::QuickJoinMenu;
use self::server_selection::ServerSelectionMenu;
use common::defaults::GAME_NAME;

pub mod hosting;
pub mod quick_join;
pub mod server_selection;

#[derive(Default)]
pub struct Menu {
    next_state: Option<NextState>,
}

impl Menu {
    pub fn new() -> Menu {
        Menu { next_state: None }
    }
}

enum NextState {
    NetworkTest,
    HostingMenu,
    ServerSelectionMenu,
    QuickJoinMenu,
}

impl Display for Menu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Menu")
    }
}

fn egui_center_width(ui: &Ui) -> f32 {
    ui.available_width() / 6.0
}

impl ProgramState for Menu {
    fn draw(
        &mut self,
        app: &mut App,
        _assets: &mut Assets,
        gfx: &mut Graphics,
        plugins: &mut Plugins,
    ) -> anyhow::Result<()> {
        let button_size = [100.0, 20.0];
        let double_button_size = [208.0, 20.0];

        let mut output = plugins.egui(|ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(GAME_NAME);
                    ui.add_space(10.0);

                    ui.vertical_centered(|ui| {
                        ui.set_width(egui_center_width(ui));
                        ui.horizontal(|ui| {
                            if ui
                                .add_sized(button_size, egui::Button::new("Quick Play"))
                                .clicked()
                            {
                                self.next_state = Some(NextState::QuickJoinMenu)
                            }

                            if ui
                                .add_sized(button_size, egui::Button::new("Join Server"))
                                .clicked()
                            {
                                self.next_state = Some(NextState::ServerSelectionMenu);
                            }
                        })
                    });

                    ui.vertical_centered(|ui| {
                        ui.set_width(egui_center_width(ui));
                        ui.horizontal(|ui| {
                            if ui
                                .add_sized(
                                    if ARGS.debug {
                                        button_size
                                    } else {
                                        double_button_size
                                    },
                                    egui::Button::new("Host Server"),
                                )
                                .clicked()
                            {
                                self.next_state = Some(NextState::HostingMenu);
                            }

                            if ARGS.debug
                                && ui
                                    .add_sized(button_size, egui::Button::new("Network Test"))
                                    .clicked()
                            {
                                self.next_state = Some(NextState::NetworkTest)
                            }
                        })
                    });

                    ui.add_space(5.0);

                    if ui
                        .add_sized(button_size, egui::Button::new("Quit"))
                        .clicked()
                    {
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
            NextState::NetworkTest => Some(NetworkTest::new().into()),
            NextState::HostingMenu => Some(HostingMenu::new().into()),
            NextState::ServerSelectionMenu => Some(ServerSelectionMenu::new().into()),
            NextState::QuickJoinMenu => Some(QuickJoinMenu::new().into()),
        }
    }
}
