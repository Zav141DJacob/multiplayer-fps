use common::defaults::PORT;
use notan::egui::{self, Ui};

use super::{Menu, SubMenu};

pub fn execute(menu: &mut Menu, ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Host Server");
        ui.add_space(10.0);

        // TODO: Center properly
        ui.horizontal(|ui| {
            ui.label("Port:");

            let response =
                ui.add(egui::TextEdit::singleline(&mut menu.port).hint_text(PORT.to_string()));

            if response.changed() {
                println!("input debug: {}", menu.port);
            }

            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                todo!()
            }
        });

        if ui.button("Host").clicked() {
            todo!()
        }

        if ui.button("Back").clicked() {
            menu.menu_state = SubMenu::Menu;
        }
    });
}
