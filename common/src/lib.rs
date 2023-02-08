use serde::{Deserialize, Serialize};

pub mod defaults;
#[derive(Serialize, Deserialize)]
pub enum FromClientMessage {
    Ping,
    Move(Direction),
    Leave,
    Join,
}

type UserID = u64;

#[derive(Serialize, Deserialize)]
pub enum FromServerMessage {
    Move(UserID, Direction),
    Join(UserID),
    Leave(UserID),
    LobbyMembers(Vec<UserID>),
    Pong,
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Direction {
    Forward,
    Backward,
    Left,
    Right,
}