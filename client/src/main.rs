mod client;
mod args;

use ::client::program::Program;
use anyhow::anyhow;
use message_io::network::RemoteAddr;
use notan::egui::EguiConfig;
use notan::prelude::WindowConfig;
use tracing_subscriber::fmt::time;
use tracing_subscriber::EnvFilter;

use notan::draw::DrawConfig;
use std::net::SocketAddr;
use crate::client::Client;
use crate::args::ARGS;

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

    // Set up up networking
    // This will be moved inside the game later, wouldn't want to connect automatically in the main menu after all
    let addr = RemoteAddr::Socket(SocketAddr::new(ARGS.ip, ARGS.port));
    println!("Starting client");

    // TODO: get channels from client
    let mut client = Client::new(addr);
    tokio::spawn(async move {
        client.run()
    });

    // Start up the windowing and game loop
    let win = WindowConfig::new()
        .vsync(true)
        // .lazy_loop(true)
        .high_dpi(true)
        .resizable(false)
        .size(1280, 720);

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
