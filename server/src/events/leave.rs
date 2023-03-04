use std::{error::Error, fmt::Display};

use hecs::NoSuchEntity;
use message_io::network::Endpoint;

use crate::server::{Logger, Server};

#[derive(Debug)]
pub enum LeaveErrors {
    FailedToRemoveFromHashMap,
    FailedToFindPlayer,
    DeSpawn(NoSuchEntity),
    Serialize(bincode::Error),
}

impl From<NoSuchEntity> for LeaveErrors {
    fn from(value: NoSuchEntity) -> Self {
        LeaveErrors::DeSpawn(value)
    }
}

impl From<bincode::Error> for LeaveErrors {
    fn from(value: bincode::Error) -> Self {
        LeaveErrors::Serialize(value)
    }
}

impl Display for LeaveErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LeaveErrors::FailedToRemoveFromHashMap => {
                write!(f, "Can't unregister a non-existent participant")
            }
            LeaveErrors::FailedToFindPlayer => write!(f, "Can't find player to despawn"),
            LeaveErrors::DeSpawn(e) => write!(f, "{e}"),
            LeaveErrors::Serialize(e) => write!(f, "{e}"),
        }
    }
}

impl Error for LeaveErrors {}

pub fn execute(server: &mut Server, endpoint: Endpoint) -> Result<(), LeaveErrors> {
    if server.is_registered(endpoint) {
        let player_entity = server
            .registered_clients
            .remove(&endpoint)
            .ok_or(LeaveErrors::FailedToRemoveFromHashMap)?;

        server.ecs.observed_world().despawn(player_entity)?;

        server.ecs.resources.get::<Logger>().unwrap().log(format!(
            "Unregistered participant with ip {}",
            endpoint.addr()
        ));
    }

    Ok(())
}
