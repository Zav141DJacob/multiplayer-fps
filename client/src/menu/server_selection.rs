use notan::egui::{self, Ui};

use super::{Menu, SubMenu};

pub fn execute(menu: &mut Menu, ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Join Server");
        ui.add_space(10.0);

        // TODO: Center properly
        ui.horizontal(|ui| {
            ui.label("Server ip and port:");

            let response = ui.text_edit_singleline(&mut menu.ip);

            if response.changed() {
                println!("input debug: {}", menu.port);
            }

            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                todo!()
            }
        });

        if ui.button("Join").clicked() {
            todo!()
        }

        if ui.button("Back").clicked() {
            menu.menu_state = SubMenu::Menu;
        }
    });
}
