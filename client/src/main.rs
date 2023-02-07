use common::{FromClientMessage, FromServerMessage};

use clap::Parser;
use hecs::World;
use message_io::network::{NetEvent, RemoteAddr, Transport};
use message_io::node::{self, NodeEvent};

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

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

const DEFAULT_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to connect to server on
    #[arg(short, long, default_value_t = 1337)]
    port: u16,

    /// IP to connect to server on
    #[arg(short, long, default_value_t = DEFAULT_IP)]
    ip: IpAddr,
}

#[tokio::main]
async fn main() {
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