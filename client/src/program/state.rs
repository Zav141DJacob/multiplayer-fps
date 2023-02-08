use std::fmt::Display;
use notan::Event;
use notan::prelude::{App, Assets, Graphics, Plugins};

use crate::game::Game;
use crate::menu::Menu;

pub trait ProgramState: Display {
    /// Return Some(ProgramState) to switch to that state
    fn update(&mut self, app: &mut App, assets: &mut Assets, plugins: &mut Plugins) {}

    fn draw(&mut self, app: &mut App, assets: &mut Assets, gfx: &mut Graphics, plugins: &mut Plugins) {}

    fn event(&mut self, app: &mut App, assets: &mut Assets, plugins: &mut Plugins, event: Event) {}

    fn change_state(&mut self, app: &mut App, assets: &mut Assets, gfx: &mut Graphics, plugins: &mut Plugins) -> Option<Box<dyn ProgramState>> {
        None
    }
}

impl<T> From<T> for Box<dyn ProgramState>
where T: ProgramState + 'static {
    fn from(value: T) -> Self {
        Box::new(value)
    }
}