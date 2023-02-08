mod client;

use ::client::program::Program;
use anyhow::anyhow;
use message_io::network::RemoteAddr;
use notan::egui::EguiConfig;
use notan::prelude::WindowConfig;
use tracing_subscriber::fmt::time;
use tracing_subscriber::EnvFilter;

use common::defaults::{IP, PORT};

use clap::Parser;

use notan::draw::DrawConfig;
use std::net::{IpAddr, SocketAddr};

use crate::client::Client;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to connect to server on
    #[arg(short, long, default_value_t = PORT)]
    port: u16,

    /// IP to connect to server on
    #[arg(short, long, default_value_t = IP)]
    ip: IpAddr,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let addr = RemoteAddr::Socket(SocketAddr::new(args.ip, args.port));
    println!("Starting client");

    // TODO: get channels from client
    let mut client = Client::new(addr);
    tokio::spawn(async move {
        client.run()
    });

    // Set up logging
    // If you want to change how the logs are filtered, then change RUST_LOG according to this:
    // https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html
    let subscriber = tracing_subscriber::fmt()
        .with_timer(time::Uptime::default())
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

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
