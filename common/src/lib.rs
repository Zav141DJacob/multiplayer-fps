use map::Map;
use serde::{Deserialize, Serialize};

pub mod defaults;
pub mod map;
pub mod ecs;

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
    Spawn(UserID, Coordinates)
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Forward,
    Backward,
    Left,
    Right,
}

pub struct Player;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Coordinates {
    x: usize,
    y: usize,
}




