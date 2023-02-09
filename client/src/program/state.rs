use std::fmt::Display;
use notan::Event;
use notan::prelude::{App, Assets, Graphics, Plugins};




pub trait ProgramState: Display {
    /// Return Some(ProgramState) to switch to that state
    fn update(&mut self, _app: &mut App, _assets: &mut Assets, _plugins: &mut Plugins) {}

    fn draw(&mut self, _app: &mut App, _assets: &mut Assets, _gfx: &mut Graphics, _plugins: &mut Plugins) {}

    fn event(&mut self, _app: &mut App, _assets: &mut Assets, _plugins: &mut Plugins, _event: Event) {}

    fn change_state(&mut self, _app: &mut App, _assets: &mut Assets, _gfx: &mut Graphics, _plugins: &mut Plugins) -> Option<Box<dyn ProgramState>> {
        None
    }
}

impl<T> From<T> for Box<dyn ProgramState>
where T: ProgramState + 'static {
    fn from(value: T) -> Self {
        Box::new(value)
    }
}