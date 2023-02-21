use common::defaults::{MAP_HEIGHT, MAP_WIDTH};
use common::ecs::utils::spawn_player;
use common::{map::Map, Coordinates};
use hecs::{Entity, World};

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

struct ClientInfo {
    id: ClientIdentificationInfo,
    entity: Option<Entity>,
}

impl ClientInfo {
    fn new(addr: SocketAddr, endpoint: Endpoint) -> Self {
        ClientInfo {
            id: ClientIdentificationInfo { addr, endpoint },
            entity: None,
        }
    }

    // TODO: Get it working
    fn set_position(&mut self, world: &mut World, coords: Coordinates) {
        // TODO: better error handling
        world
            .exchange::<common::Coordinates, common::Coordinates>(self.entity.unwrap(), coords)
            .unwrap();
    }

    fn get_position(&self, world: &mut World) -> Coordinates {
        // TODO: better error handling
        *world.get::<&Coordinates>(self.entity.unwrap()).unwrap()
    }
}
pub struct Server {
    handler: NodeHandler<()>,
    listener: Option<NodeListener<()>>,

    clients: HashMap<u64, ClientInfo>,
    world: World,
    map: Map,
}

impl Server {
    pub fn new(addr: SocketAddr) -> io::Result<Self> {
        let (handler, listener) = node::split::<()>();

        handler.network().listen(Transport::Udp, addr)?;

        Ok(Server {
            handler,
            listener: Some(listener),
            clients: HashMap::new(),
            world: World::new(),
            map: Map::gen(MAP_WIDTH, MAP_HEIGHT),
        })
    }

    pub fn run(&mut self) {
        let listener = self.listener.take().unwrap();

        listener.for_each(move |event| match event.network() {
            NetEvent::Message(endpoint, input_data) => {
                let message: FromClientMessage = bincode::deserialize(input_data).unwrap();

                let id = ClientIdentificationInfo {
                    addr: endpoint.addr(),
                    endpoint,
                };
                let name = id.get_id();

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
                        if self.is_registered(name) {
                            println!("move {direction:?}");

                            let client = self.clients.get_mut(&name).unwrap();
                            let mut coords = client.get_position(&mut self.world);

                            // TODO: get player and change position
                            match direction {
                                common::Direction::Forward => coords.x += 0.1,
                                common::Direction::Backward => coords.x -= 0.1,
                                common::Direction::Left => coords.y += 0.1,
                                common::Direction::Right => coords.y -= 0.1,
                            };

                            client.set_position(&mut self.world, coords);
                        }
                    }
                    FromClientMessage::Leave => {
                        if self.is_registered(name) {
                            self.unregister(&name)
                        }
                    }
                    FromClientMessage::Join => {
                        if !self.is_registered(name) {
                            // Registers user
                            if let Some(player_id) = self.register(id) {
                                // spawns player
                                if let Some(client) = self.clients.get_mut(&player_id) {
                                    let coords_and_entity =
                                        spawn_player(&self.map, &mut self.world, player_id);

                                    // Adds ECS entity to ClientInfo
                                    client.entity = Some(coords_and_entity.1);

                                    // TODO: handle errors better
                                    FromServerMessage::Spawn(player_id, coords_and_entity.0)
                                        .construct()
                                        .unwrap()
                                        .send(&self.handler, endpoint);
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
        self.clients.contains_key(&name)
    }

    fn register(&mut self, info: ClientIdentificationInfo) -> Option<u64> {
        let name = info.get_id();

        if !self.is_registered(name) {
            // Sends player list to the newly joined player
            let player_list = self.clients.keys().copied().collect();

            // TODO: handle errors better
            FromServerMessage::LobbyMembers(player_list)
                .construct()
                .unwrap()
                .send(&self.handler, info.endpoint);

            // Notify other players about this new player
            println!("Notifying players about new player");
            let message = FromServerMessage::Join(name).construct().unwrap();
            for participant in &mut self.clients {
                // TODO: handle errors better
                message.send(&self.handler, participant.1.id.endpoint);
            }

            // Add player to the server clients
            // TODO: replace with ECS
            println!("Added participant '{name}' with ip {}", info.addr);
            self.clients
                .insert(name, ClientInfo::new(info.addr, info.endpoint));

            // Sending initial map
            // TODO: handle errors better
            println!("Sending map to '{name}'");
            FromServerMessage::SendMap(self.map.clone())
                .construct()
                .unwrap()
                .send(&self.handler, info.endpoint);

            Some(name)
        } else {
            println!("Participant with name '{name}' already exists");

            None
        }
    }

    fn unregister(&mut self, name: &u64) {
        // TODO: Delete player from ECS

        if let Some(info) = self.clients.remove(name) {
            // Notify other participants about this removed participant
            // TODO: handle errors better
            let message = FromServerMessage::Leave(*name).construct().unwrap();
            for participant in &mut self.clients {
                message.send(&self.handler, participant.1.id.endpoint);
            }

            println!("Unregistered participant '{name}' with ip {}", info.id.addr);
        } else {
            println!("Can not unregister an non-existent participant with name '{name}'");
        }
    }
}
