use ecs::components::{EcsProtocol, InputState};
use map::Map;
use serde::{Deserialize, Serialize};

pub mod defaults;
pub mod ecs;
pub mod map;
pub mod gun;
mod maze;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FromClientMessage {
    Ping,
    Leave,
    Join(String),
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

pub enum Signal {
    Tick
}
