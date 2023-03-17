use notan::egui::Context;
use notan::egui::Id;
use notan::egui::Window;

#[derive(Clone, Default)]
pub struct ErrorWindows(pub Vec<ErrorWindow>);

impl ErrorWindows {
    pub fn new() -> ErrorWindows {
        ErrorWindows::default()
    }

    pub fn add_error(&mut self, error: String) {
        let id = match self.0.last() {
            Some(error) => error.id + 1,
            None => 0,
        };

        self.0.push(ErrorWindow::new(error, id));
    }

    /// Removes windows that have been closed
    pub fn remove_closed(&mut self) {
        self.0 = self
            .0
            .clone()
            .into_iter()
            .filter(|error| error.is_open)
            .collect();
    }

    /// Displays errors using egui windows
    pub fn draw_errors(&mut self, ctx: &Context) {
        for error in self.0.iter_mut() {
            Window::new("Error")
                .id(Id::new(error.id))
                .open(&mut error.is_open)
                .show(ctx, |ui| ui.label(error.error.to_string()));
        }

        self.remove_closed();
    }
}

#[derive(Clone)]
pub struct ErrorWindow {
    pub error: String,
    pub id: u16,
    pub is_open: bool,
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
