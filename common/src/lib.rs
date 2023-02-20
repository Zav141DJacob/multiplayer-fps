use bincode::Error;
use hecs::{Bundle};
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
    Move(UserID, Direction),
    Join(UserID),
    Leave(UserID),
    LobbyMembers(Vec<UserID>),
    SendMap(Map),
    Pong,
    Spawn(UserID, Coordinates),
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
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Forward,
    Backward,
    Left,
    Right,
}

#[derive(Bundle)]
pub struct Player;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, Bundle)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}
