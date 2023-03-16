use anyhow::anyhow;
use client::program::Program;
use common::defaults::GAME_NAME;
use notan::draw::DrawConfig;
use notan::egui::EguiConfig;
use notan::prelude::WindowConfig;
use tracing_subscriber::fmt::time;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Set up logging
    // If you want to change how the logs are filtered, then change RUST_LOG according to this:
    // https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html
    let subscriber = tracing_subscriber::fmt()
        .with_timer(time::Uptime::default())
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Start up the windowing and game loop
    #[allow(unused_mut)]
    let mut win = WindowConfig::new()
        .vsync(true)
        .title(GAME_NAME)
        // .lazy_loop(true)
        .high_dpi(false)
        .resizable(false)
        .size(1280, 720);

    #[cfg(feature = "mouse-look")]
    {
        win.fullscreen = true;
    }

    notan::init_with(Program::notan_setup)
        .add_config(win)
        .add_config(EguiConfig)
        .add_config(DrawConfig)
        .event(Program::notan_event)
        .update(Program::notan_update)
        .draw(Program::notan_draw)
        .build()
        .map_err(|str| anyhow!(str))
}
