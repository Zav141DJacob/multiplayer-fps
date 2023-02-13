mod args;
mod client;

use ::client::program::Program;
use anyhow::anyhow;
use message_io::network::RemoteAddr;
use notan::egui::EguiConfig;
use notan::prelude::WindowConfig;
use tracing_subscriber::fmt::time;
use tracing_subscriber::EnvFilter;

use crate::args::ARGS;
use crate::client::Client;
use notan::draw::DrawConfig;
use std::net::SocketAddr;

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

    let mut client = Client::new(addr);
    let (mut reciever, sender) = client.start();

    // Stop the client with this or just drop it
    // client.stop();

    // Examples behaviour for the current events
    // while let Some(message) = reciever.recv().await {
    //     match message {
    //         FromServerMessage::Pong => println!("Pong from server"),
    //         FromServerMessage::Move(id, direction) => {
    //             println!("Player {id} moved to {direction:?}")
    //         }
    //         FromServerMessage::Join(id) => {
    //             println!("Member {id} joined the lobby!")
    //         }
    //         FromServerMessage::Leave(id) => println!("Member {id} left the lobby!"),
    //         FromServerMessage::LobbyMembers(members) => {
    //             println!("current lobby members are: {members:?}")
    //         }
    //         FromServerMessage::SendMap(map) => println!("current map is: {map:?}"),
    //     }
    // }

    // Start up the windowing and game loop
    let win = WindowConfig::new()
        .vsync(true)
        // .lazy_loop(true)
        .high_dpi(true)
        .resizable(false)
        .size(720, 720);

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
