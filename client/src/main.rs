use anyhow::anyhow;
use client::program::Program;
use notan::egui::EguiConfig;
use notan::prelude::WindowConfig;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::time;


fn main() -> anyhow::Result<()> {
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






// Below is the client code that Andris wrote.
// I'll just leave it here until the server-client connection protocol is stabilized.

use common::defaults::{PORT, IP};
use common::{FromClientMessage, FromServerMessage};

use clap::Parser;
use hecs::World;
use message_io::network::{NetEvent, RemoteAddr, Transport};
use message_io::node::{self, NodeEvent};

use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use notan::draw::DrawConfig;

enum Signal {
    Greet, // This is a self event called every second.
           // Other signals here,
}

pub fn client(transport: Transport, remote_addr: RemoteAddr) {
    let (handler, listener) = node::split();

    let (server_id, local_addr) = handler
        .network()
        .connect(transport, remote_addr.clone())
        .unwrap();

    listener.for_each(move |event| match event {
        NodeEvent::Network(net_event) => match net_event {
            NetEvent::Connected(_, established) => {
                if established {
                    println!(
                        "Connected to server at {} by {}",
                        server_id.addr(),
                        transport
                    );
                    println!("Client identified by local port: {}", local_addr.port());
                    handler.signals().send(Signal::Greet);
                } else {
                    println!("Can not connect to server at {remote_addr} by {transport}")
                }
            }
            NetEvent::Accepted(_, _) => unreachable!(), // Only generated when a listener accepts
            NetEvent::Message(_, input_data) => {
                let message: FromServerMessage = bincode::deserialize(input_data).unwrap();
                match message {
                    FromServerMessage::Pong(count) => {
                        println!("Pong from server: {count} times")
                    }
                    FromServerMessage::UnknownPong => println!("Pong from server"),
                }
            }
            NetEvent::Disconnected(_) => {
                println!("Server is disconnected");
                handler.stop();
            }
        },
        NodeEvent::Signal(signal) => match signal {
            Signal::Greet => {
                let message = FromClientMessage::Ping;
                let output_data = bincode::serialize(&message).unwrap();
                handler.network().send(server_id, &output_data);
                handler
                    .signals()
                    .send_with_timer(Signal::Greet, Duration::from_secs(1));
            }
        },
    });
}

#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Backward,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct MoveEvent;

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
async fn main_old() {
    let args = Args::parse();

    let addr = SocketAddr::new(args.ip, args.port);
    println!("Starting client");

    tokio::spawn(async move {
        client(
            Transport::Udp,
            message_io::network::RemoteAddr::Socket(addr),
        );
    });

    println!("ECS system");
    let mut world = World::new();
    world.spawn((MoveEvent, Direction::Forward));

    println!(
        "{:?}",
        *world
            .query::<(&mut MoveEvent, &mut Direction)>()
            .iter()
            .next()
            .unwrap()
            .1
             .0
    );
}
