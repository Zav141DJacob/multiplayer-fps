use serde::{Deserialize, Serialize};

pub mod defaults;
pub mod map;
#[derive(Serialize, Deserialize)]
pub enum FromClientMessage {
    Ping,
    GetMap,
}

#[derive(Serialize, Deserialize)]
pub enum FromServerMessage {
    Pong(usize), // Used for connection oriented protocols
    UnknownPong, // Used for non-connection oriented protocols
}
