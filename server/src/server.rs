use common::defaults::{MAP_HEIGHT, MAP_WIDTH};
use common::ecs::components::{Player, Position, EcsProtocol};
use common::map::Map;

use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    io,
    net::SocketAddr,
};

use common::{FromClientMessage, FromServerMessage};
use message_io::{
    network::{Endpoint, NetEvent, Transport},
    node::{self, NodeHandler, NodeListener},
};

use crate::ecs::ServerEcs;
use crate::events;

#[derive(Hash)]
pub struct ClientIdentificationInfo {
    pub addr: SocketAddr,
    pub endpoint: Endpoint,
}

impl ClientIdentificationInfo {
    pub fn get_id(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);

        hasher.finish()
    }
}

pub struct ClientInfo {
    pub id: ClientIdentificationInfo,
}

impl ClientInfo {
    pub fn new(addr: SocketAddr, endpoint: Endpoint) -> Self {
        ClientInfo {
            id: ClientIdentificationInfo { addr, endpoint },
        }
    }

    fn set_position(&mut self, ecs: &mut ServerEcs, new_pos: Position) {
        let id = self.id.get_id();

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
        let id = self.id.get_id();

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
    pub handler: NodeHandler<()>,
    listener: Option<NodeListener<()>>,

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
            .map(|client| client.id.endpoint)
            .collect()
    }
}

impl Server {
    pub fn new(addr: SocketAddr) -> io::Result<Self> {
        let (handler, listener) = node::split::<()>();

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

            let protocols = self.ecs
            .observer
            .drain_reliable()
            .collect::<Vec<EcsProtocol>>();


            if protocols.len() != 0 {
                FromServerMessage::EcsChanges(
                    protocols
                )
                .construct()
                .unwrap()
                .send_all(&self.handler, self.registered_clients.get_all_endpoints());

            }
    }

    pub fn run(&mut self) {
        // self.handle_ticks();

        let listener = self.listener.take().unwrap();

            
        listener.for_each(move |event| match event.network() {
            
            NetEvent::Message(endpoint, input_data) => {

                let message: FromClientMessage = bincode::deserialize(input_data).unwrap();

                let requester_info = ClientIdentificationInfo {
                    addr: endpoint.addr(),
                    endpoint,
                };
                let requester_id = requester_info.get_id();
                self.handle_ticks();
                
                println!("Event: {message:?}");
                match message {
                    FromClientMessage::Ping => events::ping::execute(&self.handler, endpoint),
                    FromClientMessage::Leave => events::leave::execute(self, requester_id),
                    FromClientMessage::Join => {
                        events::join::execute(self, requester_id, requester_info).unwrap();
                    },
                    FromClientMessage::UpdateInputs(updated_input_state) => {
                        events::r#update_inputs::execute(self,  updated_input_state, requester_id);
                    }
                }
            }
            _ => {
                // This will never get called since we aren't using websocket
                unreachable!();
            }
        });

    }

    pub fn is_registered(&self, name: u64) -> bool {
        self.registered_clients.clients.contains_key(&name)
    }
}
