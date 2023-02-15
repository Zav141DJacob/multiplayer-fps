use common::{map::Map, Coordinates};
use hecs::World;

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
#[derive(Default)]
struct ClientStateInfo {
    position: Coordinates,
    // Something like position and other player info
}

struct ClientInfo {
    id: ClientIdentificationInfo,
    state: ClientStateInfo,
}

impl ClientInfo {
    fn new(addr: SocketAddr, endpoint: Endpoint) -> Self {
        ClientInfo {
            id: ClientIdentificationInfo { addr, endpoint },
            state: ClientStateInfo::default(),
        }
    }
    fn set_position(&mut self, coords: Coordinates) {
        self.state.position = coords;
    }
}
pub struct Server {
    handler: NodeHandler<()>,
    listener: Option<NodeListener<()>>,

    clients: HashMap<u64, ClientInfo>,
}

impl Server {
    pub fn new(addr: SocketAddr) -> io::Result<Self> {
        let (handler, listener) = node::split::<()>();

        handler.network().listen(Transport::Udp, addr)?;

        Ok(Server {
            handler,
            listener: Some(listener),
            clients: HashMap::new(),
        })
    }

    pub fn run(&mut self) {
        let listener = self.listener.take().unwrap();

        let map: Map = Map::gen();
        let mut world: World = World::new();
        listener.for_each(move |event| match event.network() {
            NetEvent::Message(endpoint, input_data) => {
                let message: FromClientMessage = bincode::deserialize(input_data).unwrap();

                let id = ClientIdentificationInfo {
                    addr: endpoint.addr(),
                    endpoint,
                };
                let name = id.get_id();

                match message {
                    FromClientMessage::Ping => {
                        println!("Ping from {}", endpoint.addr());
                        let output_data = bincode::serialize(&FromServerMessage::Pong).unwrap();
                        self.handler.network().send(endpoint, &output_data);
                    }
                    FromClientMessage::Move(_) => {
                        if self.is_registered(name) {
                            todo!()
                        }
                    },
                    FromClientMessage::Leave => {
                        if self.is_registered(name) {
                            self.unregister(&name)
                        }
                    },
                    FromClientMessage::Join => {
                        if !self.is_registered(name) {
                            //registers user
                            if let Some(player_id) = self.register(id, map.clone()) {

                                //spawns player
                                if let Some(client) = self.clients.get_mut(&player_id) {
                                    let coords = map.spawn_player(&mut world, player_id);
                                    client.set_position(coords);
                                    let output_data = bincode::serialize(&FromServerMessage::Spawn(player_id, coords)).unwrap();
                                    self.handler.network().send(endpoint, &output_data);
                                }
                            }
                        }
                    },
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

    fn register(&mut self, info: ClientIdentificationInfo, map: Map) -> Option<u64> {
        let name = info.get_id();

        if !self.is_registered(name) {
            // Sends player list to the newly joined player
            let player_list = self.clients.keys().copied().collect();
            let message = FromServerMessage::LobbyMembers(player_list);
            let output_data = bincode::serialize(&message).unwrap();
            self.handler.network().send(info.endpoint, &output_data);

            // Notify other players about this new player
            let message = FromServerMessage::Join(name);
            let output_data = bincode::serialize(&message).unwrap();
            for participant in &mut self.clients {
                self.handler
                    .network()
                    .send(participant.1.id.endpoint, &output_data);
            }
            println!("Notifying players about new player");

            // Add player to the server clients
            self.clients
                .insert(name, ClientInfo::new(info.addr, info.endpoint));
            println!("Added participant '{}' with ip {}", name, info.addr);

            // Sending initial map
            let message = FromServerMessage::SendMap(map);
            let output_data = bincode::serialize(&message).unwrap();
            self.handler.network().send(info.endpoint, &output_data);
            println!("Sending map to '{name}'");
            return Some(name)
        } else {
            println!(
                "Participant with name '{name}' already exists"
            );
            return None;
        }
    }

    fn unregister(&mut self, name: &u64) {
        if let Some(info) = self.clients.remove(name) {
            // Notify other participants about this removed participant
            let message = FromServerMessage::Leave(*name);
            let output_data = bincode::serialize(&message).unwrap();
            for participant in &mut self.clients {
                self.handler
                    .network()
                    .send(participant.1.id.endpoint, &output_data);
            }

            println!("Removed participant '{}' with ip {}", name, info.id.addr);
        } else {
            println!("Can not unregister an non-existent participant with name '{name}'");
        }
    }
}
