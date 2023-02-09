
use notan::{AppState, Event};
use notan::log::info;
use notan::prelude::{App, Assets, Graphics, Plugins};
use tracing::debug_span;
use crate::menu::Menu;
use crate::program::state::ProgramState;

pub mod state;

#[derive(AppState)]
pub struct Program {
    state: Box<dyn ProgramState>,

}

impl Default for Program {
    fn default() -> Self {
        Self {
            state: Menu::default().into(),
        }
    }
}

impl Program {
    pub fn notan_setup(_app: &mut App, _assets: &mut Assets, _gfx: &mut Graphics, _plugins: &mut Plugins) -> Self {
        Self::default()
    }

    pub fn notan_update(app: &mut App, assets: &mut Assets, plugins: &mut Plugins, this: &mut Self) {
        let span = debug_span!("update", state = %this.state);
        let _guard = span.enter();

        this.state.update(app, assets, plugins);
    }

    pub fn notan_draw(app: &mut App, assets: &mut Assets, gfx: &mut Graphics, plugins: &mut Plugins, this: &mut Self) {
        let span = debug_span!("draw", state = %this.state);
        let _guard = span.enter();

        this.state.draw(app, assets, gfx, plugins);

        // Do state switching here so we have access to Graphics (for creating textures and stuff)
        if let Some(next_state) = this.state.change_state(app, assets, gfx, plugins) {
            info!("Switched to state: {}", next_state);
            this.state = next_state;
        }
    }

    pub fn notan_event(app: &mut App, assets: &mut Assets, plugins: &mut Plugins, this: &mut Self, event: Event) {
        let span = debug_span!("event", state = %this.state);
        let _guard = span.enter();

        this.state.event(app, assets, plugins, event);
    }
}