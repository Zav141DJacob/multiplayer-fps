use common::defaults::{MAP_HEIGHT, MAP_WIDTH};
use common::ecs::components::{EcsProtocol, LookDirection, Player, Position};
use common::map::Map;
use server::ecs::ServerEcs;
use server::utils::spawn_player;

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

const PLAYER_SPEED: f32 = 0.1;

#[derive(Hash)]
struct ClientIdentificationInfo {
    addr: SocketAddr,
    endpoint: Endpoint,
}

impl ClientIdentificationInfo {
    fn get_id(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);

        hasher.finish()
    }
}

pub struct ClientInfo {
    id: ClientIdentificationInfo,
}

impl ClientInfo {
    fn new(addr: SocketAddr, endpoint: Endpoint) -> Self {
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
    handler: NodeHandler<()>,
    listener: Option<NodeListener<()>>,

    registered_clients: RegisteredClients,
    ecs: ServerEcs,
}

// Clients who have sent the join event basically
pub struct RegisteredClients {
    clients: HashMap<u64, ClientInfo>,
}

impl RegisteredClients {
    pub fn new() -> Self {
        RegisteredClients {
            clients: HashMap::new(),
        }
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

    pub fn run(&mut self) {
        let listener = self.listener.take().unwrap();

        listener.for_each(move |event| match event.network() {
            NetEvent::Message(endpoint, input_data) => {
                let message: FromClientMessage = bincode::deserialize(input_data).unwrap();

                let requester_info = ClientIdentificationInfo {
                    addr: endpoint.addr(),
                    endpoint,
                };
                let requester_id = requester_info.get_id();

                println!("Event: {message:?}");
                match message {
                    FromClientMessage::Ping => {
                        println!("Ping from {}", endpoint.addr());

                        // TODO: handle errors better
                        FromServerMessage::Pong
                            .construct()
                            .unwrap()
                            .send(&self.handler, endpoint);
                    }
                    FromClientMessage::Move(direction) => {
                        if self.is_registered(requester_id) {
                            println!("move {direction:?}");

                            let (entity, (_, look_direction, position)) = self
                                .ecs
                                .world
                                .query_mut::<(&Player, &mut LookDirection, &mut Position)>()
                                .into_iter()
                                .find(|(_, (&player, _, _))| player.id == requester_id)
                                .unwrap();

                            let mut pos = *position;
                            let dir = *look_direction;

                            let map = self.ecs.resources.get::<Map>().unwrap();
                            let w = map.get_width() as f32;
                            let h = map.get_height() as f32;

                            match direction {
                                common::Direction::Forward => {
                                    if pos.0.x + dir.0.x * PLAYER_SPEED < 0.0 {
                                        pos.0.x = 0.0;
                                    } else if pos.0.x + dir.0.x * PLAYER_SPEED > w {
                                        pos.0.x = w;
                                    } else {
                                        pos.0.x += dir.0.x * PLAYER_SPEED;
                                    }

                                    if pos.0.y - dir.0.y * PLAYER_SPEED < 0.0 {
                                        pos.0.y = 0.0;
                                    } else if pos.0.y - dir.0.y * PLAYER_SPEED > h {
                                        pos.0.y = h;
                                    } else {
                                        pos.0.y -= dir.0.y * PLAYER_SPEED;
                                    }
                                }
                                common::Direction::Backward => {
                                    if pos.0.x - dir.0.x * PLAYER_SPEED < 0.0 {
                                        pos.0.x = 0.0;
                                    } else if pos.0.x - dir.0.x * PLAYER_SPEED > w {
                                        pos.0.x = w;
                                    } else {
                                        pos.0.x -= dir.0.x * PLAYER_SPEED;
                                    }

                                    if pos.0.y + dir.0.y * PLAYER_SPEED < 0.0 {
                                        pos.0.y = 0.0;
                                    } else if pos.0.y + dir.0.y * PLAYER_SPEED > h {
                                        pos.0.y = h;
                                    } else {
                                        pos.0.y += dir.0.y * PLAYER_SPEED;
                                    }
                                }
                                common::Direction::Left => {
                                    if pos.0.x - dir.0.y * PLAYER_SPEED < 0.0 {
                                        pos.0.x = 0.0;
                                    } else if pos.0.x - dir.0.y * PLAYER_SPEED > w {
                                        pos.0.x = w;
                                    } else {
                                        pos.0.x -= dir.0.y * PLAYER_SPEED;
                                    }

                                    if pos.0.y - dir.0.x * PLAYER_SPEED < 0.0 {
                                        pos.0.y = 0.0;
                                    } else if pos.0.y - dir.0.x * PLAYER_SPEED > h {
                                        pos.0.y = h;
                                    } else {
                                        pos.0.y -= dir.0.x * PLAYER_SPEED;
                                    }
                                }
                                common::Direction::Right => {
                                    if pos.0.x + dir.0.y * PLAYER_SPEED < 0.0 {
                                        pos.0.x = 0.0;
                                    } else if pos.0.x + dir.0.y * PLAYER_SPEED > w {
                                        pos.0.x = w;
                                    } else {
                                        pos.0.x += dir.0.y * PLAYER_SPEED;
                                    }

                                    if pos.0.y + dir.0.x * PLAYER_SPEED < 0.0 {
                                        pos.0.y = 0.0;
                                    } else if pos.0.y + dir.0.x * PLAYER_SPEED > h {
                                        pos.0.y = h;
                                    } else {
                                        pos.0.y += dir.0.x * PLAYER_SPEED;
                                    }
                                }
                            };

                            *self.ecs.observer.observe_component(entity, position) = pos;
                            *self.ecs.observer.observe_component(entity, look_direction) = dir;

                            FromServerMessage::EcsChanges(
                                self.ecs
                                    .observer
                                    .drain_reliable()
                                    .collect::<Vec<EcsProtocol>>(),
                            )
                            .construct()
                            .unwrap()
                            .send_all(&self.handler, self.registered_clients.get_all_endpoints());
                        }
                    }
                    FromClientMessage::Leave => {
                        if self.is_registered(requester_id) {
                            self.unregister(requester_id)
                        }
                    }
                    FromClientMessage::Join => {
                        if !self.is_registered(requester_id) {
                            // Registers user
                            // TODO: clean up this part
                            if let Some(player_id) = self.register(requester_info) {
                                // spawns player
                                if self
                                    .registered_clients
                                    .clients
                                    .get_mut(&player_id)
                                    .is_some()
                                {
                                    spawn_player(&mut self.ecs, player_id);

                                    // TODO: handle errors better
                                    FromServerMessage::EcsChanges(
                                        self.ecs
                                            .observer
                                            .drain_reliable()
                                            .collect::<Vec<EcsProtocol>>(),
                                    )
                                    .construct()
                                    .unwrap()
                                    .send_all(
                                        &self.handler,
                                        self.registered_clients.get_all_endpoints(),
                                    );
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                // This will never get called since we aren't using websocket
                unreachable!();
            }
        });
    }

    fn is_registered(&self, name: u64) -> bool {
        self.registered_clients.clients.contains_key(&name)
    }

    fn register(&mut self, info: ClientIdentificationInfo) -> Option<u64> {
        let id = info.get_id();

        if !self.is_registered(id) {
            // Sends player list to the newly joined player
            let player_list = self.registered_clients.clients.keys().copied().collect();

            // TODO: handle errors better
            FromServerMessage::LobbyMembers(player_list)
                .construct()
                .unwrap()
                .send(&self.handler, info.endpoint);

            // Notify other players about this new player joining the game server
            println!("Notifying players about new player");
            FromServerMessage::Join(id)
                .construct()
                .unwrap()
                .send_all(&self.handler, self.registered_clients.get_all_endpoints());

            // Add player to the server clients
            println!("Added participant '{id}' with ip {}", info.addr);
            self.registered_clients
                .clients
                .insert(id, ClientInfo::new(info.addr, info.endpoint));

            // Sending initial map to player
            // TODO: handle errors better
            println!("Sending map to '{id}'");
            FromServerMessage::SendMap(self.ecs.resources.get::<Map>().unwrap().clone())
                .construct()
                .unwrap()
                .send(&self.handler, info.endpoint);

            Some(id)
        } else {
            println!("Participant with name '{id}' already exists");

            None
        }
    }

    fn unregister(&mut self, id: u64) {
        if let Some(info) = self.registered_clients.clients.remove(&id) {
            // TODO: fix it not sending leave message
            // Notifies other participants about this removed participant
            let entity = self
                .ecs
                .world
                .query::<&Player>()
                .iter()
                .find(|(_, &player)| player.id == id)
                .unwrap()
                .0;
            self.ecs.observed_world().despawn(entity).unwrap();

            FromServerMessage::EcsChanges(
                self.ecs
                    .observer
                    .drain_reliable()
                    .collect::<Vec<EcsProtocol>>(),
            )
            .construct()
            .unwrap()
            .send_all(&self.handler, self.registered_clients.get_all_endpoints());

            println!("Unregistered participant '{id}' with ip {}", info.id.addr);
        } else {
            println!("Can not unregister an non-existent participant with name '{id}'");
        }
    }
}
