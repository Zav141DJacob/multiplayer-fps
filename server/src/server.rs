use common::defaults::{MAP_HEIGHT, MAP_WIDTH, TICKS_PER_SECOND};
use common::ecs::components::{EcsProtocol, Player, Position};
use common::map::Map;
use message_io::node::NodeEvent;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use std::fmt::Display;
use std::time::Duration;
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    io,
    net::SocketAddr,
};

use common::{FromClientMessage, FromServerMessage, Signal};
use message_io::{
    network::{Endpoint, NetEvent, Transport},
    node::{self, NodeHandler, NodeListener},
};

use crate::ecs::ServerEcs;
use crate::events;

#[derive(Hash)]
pub struct ClientInfo {
    pub addr: SocketAddr,
    pub endpoint: Endpoint,
}

impl ClientInfo {
    pub fn get_id(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);

        hasher.finish()
    }

    pub fn new(addr: SocketAddr, endpoint: Endpoint) -> Self {
        ClientInfo { addr, endpoint }
    }

    fn set_position(&mut self, ecs: &mut ServerEcs, new_pos: Position) {
        let id = self.get_id();

        // TODO: better error handling
        let (entity, (_, position)) = ecs
            .world
            .query_mut::<(&Player, &mut Position)>()
            .into_iter()
            .find(|(_, (&player, _))| player.id == id)
            .unwrap();

        *ecs.observer.observe_component(entity, position) = new_pos;
    }

    fn get_position(&self, ecs: &ServerEcs) -> Position {
        let id = self.get_id();

        // TODO: better error handling
        *ecs.world
            .query::<(&Position, &Player)>()
            .iter()
            .find(|(_, (_, &player))| player.id == id)
            .unwrap()
            .1
             .0
    }
}

pub struct Server {
    pub handler: NodeHandler<Signal>,
    listener: Option<NodeListener<Signal>>,

    pub registered_clients: RegisteredClients,
    pub ecs: ServerEcs,
}

#[derive(Clone)]
pub struct Logger {
    pub sender: UnboundedSender<String>,
    pub enable_channels: bool,
}

impl Logger {
    pub fn new(enable_channels: bool) -> (Logger, UnboundedReceiver<String>) {
        let (sender, reciever) = mpsc::unbounded_channel::<String>();

        (
            Logger {
                sender,
                enable_channels,
            },
            reciever,
        )
    }

    pub fn log<T: Display>(&self, message: T) {
        let msg = format!("Server: {message}");

        if self.enable_channels && self.sender.send(msg.clone()).is_err() {
            println!("Warning: failed to send message to channel");
        }

        println!("{msg}");
    }
}

// Clients who have sent the join event basically
#[derive(Default)]
pub struct RegisteredClients {
    pub clients: HashMap<u64, ClientInfo>,
}

impl RegisteredClients {
    pub fn new() -> Self {
        RegisteredClients::default()
    }

    pub fn get_all_endpoints(&self) -> Vec<Endpoint> {
        self.clients
            .values()
            .map(|client| client.endpoint)
            .collect()
    }
}

impl Server {
    pub fn new(
        addr: SocketAddr,
        enable_logging_channels: bool,
    ) -> io::Result<(Self, UnboundedReceiver<String>)> {
        let (handler, listener) = node::split::<Signal>();

        handler.network().listen(Transport::Udp, addr)?;

        let mut ecs = ServerEcs::default();
        ecs.resources.insert(Map::gen(MAP_WIDTH, MAP_HEIGHT));
        let (logger, logger_reciever) = Logger::new(enable_logging_channels);
        ecs.resources.insert(logger);

        Ok((
            Server {
                handler,
                listener: Some(listener),
                registered_clients: RegisteredClients::new(),
                ecs,
            },
            logger_reciever,
        ))
    }

    pub fn handle_ticks(&mut self) {
        self.ecs.tick(0.0);

        let protocols = self
            .ecs
            .observer
            .drain_reliable()
            .collect::<Vec<EcsProtocol>>();

        if !protocols.is_empty() {
            FromServerMessage::EcsChanges(protocols)
                .construct()
                .unwrap()
                .send_all(&self.handler, self.registered_clients.get_all_endpoints());
        }
        self.handler
            .signals()
            .send_with_timer(Signal::Tick, Duration::from_millis(1000 / TICKS_PER_SECOND));
    }

    pub fn run(&mut self) {
        self.handle_ticks();
        let logger = self.ecs.resources.get::<Logger>().unwrap().clone();
        let listener = self.listener.take().unwrap();

        listener.for_each(move |event| {
            match event {
                NodeEvent::Signal(signal) => match signal {
                    Signal::Tick => {
                        self.handle_ticks();
                    }
                    _ => (), // I put the Signal enum inside common, so I would like some input on
                             // if we should merge Signals from client as well
                },
                NodeEvent::Network(net_event) => match net_event {
                    NetEvent::Message(endpoint, input_data) => {
                        let message: FromClientMessage = bincode::deserialize(input_data).unwrap();

                        let requester_info = ClientInfo {
                            addr: endpoint.addr(),
                            endpoint,
                        };
                        let requester_id = requester_info.get_id();

                        logger.log(format!("Event {message:?}"));

                        match message {
                            FromClientMessage::Ping => {
                                events::ping::execute(&logger, &self.handler).unwrap()
                            }
                            FromClientMessage::Leave => {
                                events::leave::execute(self, requester_id).unwrap()
                            }
                            FromClientMessage::Join => {
                                events::join::execute(self, requester_id, requester_info).unwrap();
                            }
                            FromClientMessage::UpdateInputs(updated_input_state) => {
                                events::r#update_inputs::execute(
                                    self,
                                    updated_input_state,
                                    requester_id,
                                );
                            }
                        }
                    }
                    _ => (),
                },
            }
        });
    }

    pub fn is_registered(&self, name: u64) -> bool {
        self.registered_clients.clients.contains_key(&name)
    }
}
