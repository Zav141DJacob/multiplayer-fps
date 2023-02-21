use bincode::Error;
use ecs::components::EcsProtocol;
use map::Map;
use message_io::{network::Endpoint, node::NodeHandler};
use serde::{Deserialize, Serialize};

pub mod defaults;
pub mod ecs;
pub mod map;

#[derive(Debug, Serialize, Deserialize)]
pub enum FromClientMessage {
    Ping,
    Move(Direction),
    Leave,
    Join,
}

pub type UserID = u64;

#[derive(Debug, Serialize, Deserialize)]
pub enum FromServerMessage {
    Join(UserID), // TODO: look into if this is still needed in the future
    LobbyMembers(Vec<UserID>), // TODO: look into if this is still needed in the future
    SendMap(Map),
    Pong,
    EcsChanges(Vec<EcsProtocol>)
}

impl FromServerMessage {
    pub fn construct(&self) -> Result<ConstructedMessage, Error> {
        Ok(ConstructedMessage(bincode::serialize(self)?))
    }
}

pub struct ConstructedMessage(Vec<u8>);

impl ConstructedMessage {
    pub fn send(&self, handler: &NodeHandler<()>, endpoint: Endpoint) {
        handler.network().send(endpoint, &self.0);
    }

    pub fn send_all(&self, handler: &NodeHandler<()>, endpoints: Vec<Endpoint>) {
        for endpoint in endpoints {
            self.send(handler, endpoint)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Forward,
    Backward,
    Left,
    Right,
}
