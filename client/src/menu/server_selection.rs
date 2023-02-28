use notan::egui::{self, Ui};

use super::{Menu, SubMenu};

// TODO: make game use specified connection settings
pub fn execute(menu: &mut Menu, ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Join Server");
        ui.add_space(10.0);

        // TODO: Center properly
        ui.horizontal(|ui| {
            ui.label("Server ip and port:");

            let response = ui.text_edit_singleline(&mut menu.ip);

            // TODO: input validation
            if response.changed() {
                println!("input debug: {}", menu.port);
            }

            // When you press enter it submits
            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                menu.next_state = Some(super::NextState::Game);
            }
        });

        if ui.button("Join").clicked() {
            menu.next_state = Some(super::NextState::Game);
        }

        if ui.button("Back").clicked() {
            menu.menu_state = SubMenu::Menu;
        }
    });
}
