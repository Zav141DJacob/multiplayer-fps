use common::defaults::{MAP_HEIGHT, MAP_WIDTH, TICKS_PER_SECOND};
use common::ecs::components::{EcsProtocol, Player, Position};
use common::map::Map;
use message_io::node::NodeEvent;

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
    pub fn new(addr: SocketAddr) -> io::Result<Self> {
        let (handler, listener) = node::split::<Signal>();

        handler.network().listen(Transport::Udp, addr)?;

        let mut ecs = ServerEcs::default();
        ecs.resources.insert(Map::gen(MAP_WIDTH, MAP_HEIGHT));

        Ok(Server {
            handler,
            listener: Some(listener),
            registered_clients: RegisteredClients::new(),
            ecs,
        })
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

        let listener = self.listener.take().unwrap();

        listener.for_each(move |event| {

            match event {
                NodeEvent::Signal(signal) => match signal {
                    Signal::Tick => {
                        self.handle_ticks();
                    }, 
                    _ => ()
                    // I put the Signal enum inside common, so I would like some input on 
                    // if we should merge Signals from client as well
                },
                NodeEvent::Network(net_event) => {
                    match net_event {
                        NetEvent::Message(endpoint, input_data) => {
                            let message: FromClientMessage = bincode::deserialize(input_data).unwrap();
        
                            let requester_info = ClientInfo {
                                addr: endpoint.addr(),
                                endpoint,
                            };
                            let requester_id = requester_info.get_id();
        
                            println!("Event: {message:?}");
                            match message {
                                FromClientMessage::Ping => events::ping::execute(&self.handler, endpoint),
                                FromClientMessage::Leave => events::leave::execute(self, requester_id),
                                FromClientMessage::Join => {
                                    events::join::execute(self, requester_id, requester_info).unwrap();
                                }
                                FromClientMessage::UpdateInputs(updated_input_state) => {
                                    events::r#update_inputs::execute(self, updated_input_state, requester_id);
                                }
                            }
                        },
                        _ => ()
                    }
                }
                
                
            }
        });
    }

    pub fn is_registered(&self, name: u64) -> bool {
        self.registered_clients.clients.contains_key(&name)
    }
}
