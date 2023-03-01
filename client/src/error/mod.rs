use std::error::Error;
use std::fmt::{Display, Formatter};

use notan::app::{App, Graphics, Plugins};
use notan::egui::{self, Align2, EguiPluginSugar};
use notan::prelude::{Assets, Color};

use crate::menu::Menu;
use crate::program::state::ProgramState;

#[derive(Default)]
pub struct ErrorState {
    title: String,
    message: String,
    exit: bool,
}

impl Display for ErrorState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ErrorState")
    }
}

impl ErrorState {
    pub fn new(title: String, message: impl Display) -> Self {
        Self {
            title,
            message: message.to_string(),
            exit: false,
        }
    }
}

impl<E> From<E> for ErrorState
where E: Error {
    fn from(err: E) -> Self {
        ErrorState::new("Error".to_string(), err.to_string())
    }
}

impl ProgramState for ErrorState {
    fn draw(
        &mut self,
        _app: &mut App,
        _assets: &mut Assets,
        gfx: &mut Graphics,
        plugins: &mut Plugins,
    ) -> anyhow::Result<()> {
        let mut output = plugins.egui(|ctx| {
            let window = egui::Window::new(&self.title)
                .resizable(false)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0]);

            window.show(ctx, |ui| ui.vertical_centered(|ui| {
                ui.add_space(6.0);
                ui.label(&self.message);
                ui.add_space(6.0);

                if ui.button("Return").clicked() {
                    self.exit = true;
                }
            }));
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
        self.exit.then(|| Menu::default().into())
    }
}
