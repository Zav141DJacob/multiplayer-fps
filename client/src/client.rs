use std::time::Duration;

use common::{FromClientMessage, FromServerMessage};
use message_io::{
    network::{NetEvent, RemoteAddr, Transport},
    node::{self, NodeEvent},
};

enum Signal {
    Greet, // This is a self event called every second.
           // Other signals here,
}

pub fn run(remote_addr: RemoteAddr) {
    let (handler, listener) = node::split();

    let (server_id, local_addr) = handler
        .network()
        .connect(Transport::Udp, remote_addr.clone())
        .unwrap();

    // Sends join event
    handler.network().send(
        server_id,
        &bincode::serialize(&FromClientMessage::Join).unwrap(),
    );

    listener.for_each(move |event| match event {
        NodeEvent::Network(net_event) => match net_event {
            NetEvent::Connected(_, established) => {
                if established {
                    println!("Connected to server at {}", server_id.addr(),);
                    println!("Client identified by local port: {}", local_addr.port());
                    handler.signals().send(Signal::Greet);
                } else {
                    println!("Can not connect to server at {remote_addr}")
                }
            }
            NetEvent::Accepted(_, _) => unreachable!(), // Only generated when a listener accepts
            NetEvent::Message(_, input_data) => {
                let message: FromServerMessage = bincode::deserialize(input_data).unwrap();
                match message {
                    FromServerMessage::Pong => println!("Pong from server"),
                    FromServerMessage::Move(id, direction) => {
                        println!("Player {id} moved to {direction:?}")
                    }
                    FromServerMessage::Join(id) => println!("Member {id} joined the lobby!"),
                    FromServerMessage::Leave(id) => println!("Member {id} left the lobby!"),
                    FromServerMessage::LobbyMembers(members) => {
                        println!("current lobby members are: {members:?}")
                    }
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
