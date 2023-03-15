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

    pub fn remove_closed(&mut self) {
        self.0 = self
            .0
            .clone()
            .into_iter()
            .filter(|error| error.is_open)
            .collect();
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
