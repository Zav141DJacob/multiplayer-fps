use common::defaults::PORT;
use notan::egui::{self, Ui};

use super::{Menu, SubMenu};

pub fn host_server(port: &str) {
    // TODO: complete server hosting part
    // cargo run --release --bin server --port 1337
    // server::run_server("127.0.0.1".parse().unwrap(), port.parse().unwrap()).unwrap()
}

// TODO: make game use specified connection settings
pub fn execute(menu: &mut Menu, ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Host Server");
        ui.add_space(10.0);

        ui.vertical_centered(|ui| {
            ui.set_width(ui.available_width() / 4.0);
            ui.horizontal(|ui| {
                ui.label("Port:");

                let response =
                    ui.add(egui::TextEdit::singleline(&mut menu.port).hint_text(PORT.to_string()));

                // TODO: input validation
                if response.changed() {
                    println!("input debug: {}", menu.port);
                }

                // When you press enter it submits
                if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                    host_server(&menu.port);
                    menu.next_state = Some(super::NextState::Game);
                }

                if ui.button("Host").clicked() {
                    host_server(&menu.port);
                    menu.next_state = Some(super::NextState::Game);
                }
            })
        });

        ui.add_space(5.0);
        if ui.button("Back").clicked() {
            menu.menu_state = SubMenu::Menu;
        }
    });
}
