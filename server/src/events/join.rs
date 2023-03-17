use common::defaults::DEFAULT_PLAYER_NAME;
use message_io::network::Endpoint;
use std::{error::Error, fmt::Display};

use common::{map::Map, FromServerMessage};
use resources::CantGetResource;

use crate::constructed_message::ConstructMessage;
use crate::ecs::spawn::player::spawn_player;
use crate::server::{Logger, Server};

#[derive(Debug)]
pub enum JoinError {
    Resources(CantGetResource),
    Serialize(bincode::Error),
}

impl From<bincode::Error> for JoinError {
    fn from(value: bincode::Error) -> Self {
        JoinError::Serialize(value)
    }
}

impl From<CantGetResource> for JoinError {
    fn from(value: CantGetResource) -> Self {
        JoinError::Resources(value)
    }
}

impl Error for JoinError {}

impl Display for JoinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JoinError::Resources(e) => write!(f, "JoinError: {e}"),
            JoinError::Serialize(e) => write!(f, "JoinError: {e}"),
        }
    }
}
// Registers user
pub fn execute(server: &mut Server, endpoint: Endpoint, username: &str) -> Result<(), JoinError> {
    let logger = server.ecs.resources.get::<Logger>().unwrap().clone();

    if server.is_registered(endpoint) {
        logger.log(format!(
            "Participant with IP {} already exists",
            endpoint.addr()
        ));
        return Ok(());
    }

    let (_, entity) = spawn_player(
        &mut server.ecs,
        if username.is_empty() {
            DEFAULT_PLAYER_NAME
        } else {
            username
        },
    );

    FromServerMessage::OwnId(entity.to_bits().into())
        .construct()?
        .send(&server.handler, endpoint);

    // Add player to the server clients
    logger.log(format!("Added participant with ip {}", endpoint.addr()));

    // Spawns player

    server.registered_clients.insert(endpoint, entity);

    // Sending initial map to player
    logger.log(format!("Sending map to IP {}", endpoint.addr()));

    FromServerMessage::SendMap(server.ecs.resources.get::<Map>()?.clone())
        .construct()?
        .send(&server.handler, endpoint);

    // Sends ECS history to the newly joined user
    FromServerMessage::EcsChanges(server.ecs.init_client())
        .construct()?
        .send(&server.handler, endpoint);

    Ok(())
}
