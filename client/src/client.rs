use std::{net::SocketAddr, time::Duration};

use common::{FromClientMessage, FromServerMessage};
use message_io::{
    network::{Endpoint, NetEvent, RemoteAddr, Transport},
    node::{self, NodeEvent, NodeHandler, NodeListener},
};

enum Signal {
    Greet, // This is a self event called every second.
           // Other signals here,
}

pub struct Client {
    handler: NodeHandler<Signal>,
    listener: Option<NodeListener<Signal>>,

    server_id: Endpoint,
    local_addr: SocketAddr,
}

impl Client {
    pub fn new(remote_addr: RemoteAddr) -> Self {
        let (handler, listener) = node::split();

        let (server_id, local_addr) = handler
            .network()
            .connect(Transport::Udp, remote_addr)
            .unwrap();

        Client {
            handler,
            listener: Some(listener),
            server_id,
            local_addr,
        }
    }

    pub fn run(&mut self) {
        // Sends join event
        self.handler.network().send(
            self.server_id,
            &bincode::serialize(&FromClientMessage::Join).unwrap(),
        );

        // TODO: send disconnect event somewhere
        let listener = self.listener.take().unwrap();
        listener.for_each(move |event| match event {
            NodeEvent::Network(net_event) => match net_event {
                NetEvent::Connected(_, established) => {
                    if established {
                        println!("Connected to server at {}", self.server_id.addr(),);
                        println!(
                            "Client identified by local port: {}",
                            self.local_addr.port()
                        );
                        self.handler.signals().send(Signal::Greet);
                    } else {
                        println!("Cant connect to server")
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
                        FromServerMessage::SendMap(map) => println!("current map is: {map:?}"),
                    }
                }
                NetEvent::Disconnected(_) => {
                    println!("Server is disconnected");
                    self.handler.stop();
                }
            },
            NodeEvent::Signal(signal) => match signal {
                Signal::Greet => {
                    let message = FromClientMessage::Ping;
                    let output_data = bincode::serialize(&message).unwrap();
                    self.handler.network().send(self.server_id, &output_data);
                    self.handler
                        .signals()
                        .send_with_timer(Signal::Greet, Duration::from_secs(1));
                }
            },
        });
    }
}
