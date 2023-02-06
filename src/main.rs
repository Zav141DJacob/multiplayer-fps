use std::net::{SocketAddr, IpAddr, Ipv4Addr};

use clap::Parser;
use hecs::{World};
use message_io::network::Transport;
use crate::server::server;
use crate::client::client;

mod common;
mod client;
mod server;

const default_ip: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// If to host server
    #[arg(long, default_value_t = false)]
    host: bool,

    /// Disables client
    #[arg(long, default_value_t = false)]
    disable_client: bool,

    /// Port to connect to server or host server on
    #[arg(short, long, default_value_t = 1337)]
    port: u16,

    /// IP to connect to server or host server on
    #[arg(short, long, default_value_t = default_ip)]
    ip: IpAddr,
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

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let addr: SocketAddr = SocketAddr::new(args.ip, args.port);
    if args.host {
        println!("Starting server");

        tokio::spawn(async move {
            server(Transport::Udp, addr);
        });
    }

    if !args.disable_client {
        println!("Starting client");

        tokio::spawn(async move {
            client(Transport::Udp, message_io::network::RemoteAddr::Socket(addr));
        });
    }

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
