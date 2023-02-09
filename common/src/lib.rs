use map::Map;
use serde::{Deserialize, Serialize};

pub mod defaults;
pub mod map;
#[derive(Debug, Serialize, Deserialize)]
pub enum FromClientMessage {
    Ping,
    Move(Direction),
    Leave,
    Join,
}

type UserID = u64;

#[derive(Debug, Serialize, Deserialize)]
pub enum FromServerMessage {
    Move(UserID, Direction),
    Join(UserID),
    Leave(UserID),
    LobbyMembers(Vec<UserID>),
    SendMap(Map),
    Pong,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Direction {
    Forward,
    Backward,
    Left,
    Right,
}
