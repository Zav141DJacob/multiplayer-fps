use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use clap::Parser;
use message_io::{
    network::{Endpoint, NetEvent, Transport},
    node,
};

use common::{FromClientMessage, FromServerMessage};

struct ClientInfo {
    count: usize,
}

pub fn server(transport: Transport, addr: SocketAddr) {
    let (handler, listener) = node::split::<()>();

    let mut clients: HashMap<Endpoint, ClientInfo> = HashMap::new();

    match handler.network().listen(transport, addr) {
        Ok((_id, real_addr)) => println!("Server running at {real_addr} by {transport}"),
        Err(_) => return println!("Can not listening at {addr} by {transport}"),
    }

    listener.for_each(move |event| match event.network() {
        NetEvent::Connected(_, _) => (), // Only generated at connect() calls.
        NetEvent::Accepted(endpoint, _listener_id) => {
            // Only connection oriented protocols will generate this event
            clients.insert(endpoint, ClientInfo { count: 0 });
            println!(
                "Client ({}) connected (total clients: {})",
                endpoint.addr(),
                clients.len()
            );
        }
        NetEvent::Message(endpoint, input_data) => {
            let message: FromClientMessage = bincode::deserialize(input_data).unwrap();
            match message {
                FromClientMessage::Ping => {
                    let message = match clients.get_mut(&endpoint) {
                        Some(client) => {
                            // For connection oriented protocols
                            client.count += 1;
                            println!("Ping from {}, {} times", endpoint.addr(), client.count);
                            FromServerMessage::Pong(client.count)
                        }
                        None => {
                            // For non-connection oriented protocols
                            println!("Ping from {}", endpoint.addr());
                            FromServerMessage::UnknownPong
                        }
                    };
                    let output_data = bincode::serialize(&message).unwrap();
                    handler.network().send(endpoint, &output_data);
                }
            }
        }
        NetEvent::Disconnected(endpoint) => {
            // Only connection oriented protocols will generate this event
            clients.remove(&endpoint).unwrap();
            println!(
                "Client ({}) disconnected (total clients: {})",
                endpoint.addr(),
                clients.len()
            );
        }
    });
}

const DEFAULT_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to host server on
    #[arg(short, long, default_value_t = 1337)]
    port: u16,

    /// IP to host server on
    #[arg(short, long, default_value_t = DEFAULT_IP)]
    ip: IpAddr,
}

fn main() {
    todo!()
}
