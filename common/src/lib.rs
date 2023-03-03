use ecs::components::{EcsProtocol, InputState};
use map::Map;
use message_io::{network::Endpoint, node::NodeHandler};
use serde::{Deserialize, Serialize};

pub mod defaults;
pub mod ecs;
pub mod map;

#[derive(Debug, Serialize, Deserialize)]
pub enum FromClientMessage {
    Ping,
    // Move(Direction),
    Leave,
    Join,
    UpdateInputs(InputState)
}

pub type UserID = u64;

#[derive(Debug, Serialize, Deserialize)]
pub enum FromServerMessage {
    OwnId(UserID),
    SendMap(Map),
    Pong,
    EcsChanges(Vec<EcsProtocol>)
}

impl FromServerMessage {
    pub fn construct(&self) -> Result<ConstructedMessage, bincode::Error> {
        Ok(ConstructedMessage(bincode::serialize(self)?))
    }
}
pub enum Signal {
    Tick
}
pub struct ConstructedMessage(Vec<u8>);

impl ConstructedMessage {
    pub fn send(&self, handler: &NodeHandler<Signal>, endpoint: Endpoint) {
        handler.network().send(endpoint, &self.0);
    }

    pub fn send_all(&self, handler: &NodeHandler<Signal>, endpoints: Vec<Endpoint>) {
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


