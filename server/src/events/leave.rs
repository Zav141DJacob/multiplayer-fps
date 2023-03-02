use std::{error::Error, fmt::Display};

use common::{
    ecs::components::{EcsProtocol, Player},
    FromServerMessage,
};
use hecs::NoSuchEntity;

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

pub fn execute(server: &mut Server, requester_id: u64) -> Result<(), LeaveErrors> {
    if server.is_registered(requester_id) {
        let requester_info = server
            .registered_clients
            .clients
            .remove(&requester_id)
            .ok_or(LeaveErrors::FailedToRemoveFromHashMap)?;

        // Notifies other participants about this removed participant
        let player_entity = server
            .ecs
            .world
            .query::<&Player>()
            .iter()
            .find(|(_, &player)| player.id == requester_id)
            .ok_or(LeaveErrors::FailedToFindPlayer)?
            .0;
        server.ecs.observed_world().despawn(player_entity)?;

        FromServerMessage::EcsChanges(
            server
                .ecs
                .observer
                .drain_reliable()
                .collect::<Vec<EcsProtocol>>(),
        )
        .construct()?
        .send_all(
            &server.handler,
            server.registered_clients.get_all_endpoints(),
        );

        server.ecs.resources.get::<Logger>().unwrap().log(format!(
            "Unregistered participant '{requester_id}' with ip {}",
            requester_info.addr
        ));
    }

    Ok(())
}
