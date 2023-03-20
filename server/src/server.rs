use chrono::Utc;
use common::defaults::{MAP_HEIGHT, MAP_WIDTH, TICKS_PER_SECOND};
use common::ecs::components::EcsProtocol;
use common::map::Map;
use message_io::node::NodeEvent;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use hecs::Entity;
use std::collections::HashMap;
use std::fmt::Display;
use std::io;
use std::net::SocketAddr;
use std::time::{Duration, Instant};

use crate::constructed_message::ConstructMessage;
use crate::ecs::spawn::weapon_crate::spawn_weapon_crates_init;
use common::{FromClientMessage, FromServerMessage, Signal};
use message_io::{
    network::{Endpoint, NetEvent, Transport},
    node::{self, NodeHandler, NodeListener},
};

use crate::ecs::ServerEcs;
use crate::events;

pub struct Server {
    last_tick: Instant,

    pub handler: NodeHandler<Signal>,
    listener: Option<NodeListener<Signal>>,

    pub registered_clients: RegisteredClients,
    pub ecs: ServerEcs,
}

/// Maps endpoints to their player entity
pub type RegisteredClients = HashMap<Endpoint, Entity>;

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
        let datetime = Utc::now().format("%Y-%m-%d %H:%M:%S");
        let msg = format!("[{datetime}]: {message}");

        if self.enable_channels && self.sender.send(msg.clone()).is_err() {
            println!("Warning: failed to send message to channel");
        }

        println!("{msg}");
    }
}

// Clients who have sent the join event basically

impl Server {
    pub fn new(
        addr: SocketAddr,
        enable_logging_channels: bool,
    ) -> io::Result<(Self, UnboundedReceiver<String>)> {
        let (handler, listener) = node::split::<Signal>();

        handler.network().listen(Transport::Udp, addr)?;

        let mut ecs = ServerEcs::default();
        ecs.resources.insert(Map::gen(MAP_WIDTH, MAP_HEIGHT));
        spawn_weapon_crates_init(&mut ecs);
        let (logger, logger_receiver) = Logger::new(enable_logging_channels);
        ecs.resources.insert(logger);

        Ok((
            Server {
                last_tick: Instant::now(),
                handler,
                listener: Some(listener),
                registered_clients: RegisteredClients::new(),
                ecs,
            },
            logger_receiver,
        ))
    }

    pub fn handle_ticks(&mut self) {
        let dt = self.last_tick.elapsed().as_secs_f32();
        self.last_tick = Instant::now();
        self.ecs.tick(dt);

        let protocols = self
            .ecs
            .observer
            .drain_reliable()
            .collect::<Vec<EcsProtocol>>();

        if !protocols.is_empty() {
            FromServerMessage::EcsChanges(protocols)
                .construct()
                .unwrap()
                .send_all(&self.handler, &self.registered_clients);
        }
        self.handler
            .signals()
            .send_with_timer(Signal::Tick, Duration::from_millis(1000 / TICKS_PER_SECOND));
    }

    pub fn run(&mut self) {
        self.handle_ticks();
        let logger = self.ecs.resources.get::<Logger>().unwrap().clone();
        let listener = self.listener.take().unwrap();

        listener.for_each(move |event| match event {
            NodeEvent::Signal(signal) => match signal {
                Signal::Tick => {
                    self.handle_ticks();
                }
            },
            NodeEvent::Network(net_event) => {
                if let NetEvent::Message(endpoint, input_data) = net_event {
                    let message: FromClientMessage = match bincode::deserialize(input_data) {
                        Ok(m) => m,
                        Err(_) => {
                            logger.log("Warning: Invalid message sent to server");
                            return;
                        }
                    };

                    // logger.log(format!("Event {message:?}"));

                    match message {
                        FromClientMessage::Ping => {
                            events::ping::execute(&logger, &self.handler, endpoint).unwrap()
                        }
                        FromClientMessage::Leave => events::leave::execute(self, endpoint).unwrap(),
                        FromClientMessage::Join(username) => {
                            events::join::execute(self, endpoint, &username).unwrap();
                        }
                        FromClientMessage::UpdateInputs(updated_input_state) => {
                            if let Err(err) =
                                events::update_inputs::execute(self, updated_input_state, endpoint)
                            {
                                logger.log(format!("Warning: {err}"))
                            }
                        }
                    }
                }
            }
        });
    }

    pub fn is_registered(&self, endpoint: Endpoint) -> bool {
        self.registered_clients.contains_key(&endpoint)
    }
}
