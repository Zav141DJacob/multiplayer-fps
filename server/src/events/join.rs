use std::{error::Error, fmt::Display};

use common::{ecs::components::EcsProtocol, map::Map, FromServerMessage};
use resources::CantGetResource;

use crate::{
    server::{ClientIdentificationInfo, ClientInfo, Server},
    utils::spawn_player,
};

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
pub fn execute(
    server: &mut Server,
    requester_id: u64,
    requester_info: ClientIdentificationInfo,
) -> Result<(), JoinError> {
    if !server.is_registered(requester_id) {
        // Sends player list to the newly joined player
        FromServerMessage::LobbyMembers(
            server.registered_clients.clients.keys().copied().collect(),
        )
        .construct()?
        .send(&server.handler, requester_info.endpoint);


        FromServerMessage::OwnId(requester_id).construct()?.send(
            &server.handler,
            requester_info.endpoint,
        );


        // Notify other players about this new player joining the game server
        println!("Notifying players about new player");
        FromServerMessage::Join(requester_id).construct()?.send_all(
            &server.handler,
            server.registered_clients.get_all_endpoints(),
        );

        // Add player to the server clients
        println!(
            "Added participant '{requester_id}' with ip {}",
            requester_info.addr
        );
        server.registered_clients.clients.insert(
            requester_id,
            ClientInfo::new(requester_info.addr, requester_info.endpoint),
        );

        // Sending initial map to player
        // TODO: handle errors better
        println!("Sending map to '{requester_id}'");
        FromServerMessage::SendMap(server.ecs.resources.get::<Map>()?.clone())
            .construct()?
            .send(&server.handler, requester_info.endpoint);

        FromServerMessage::EcsChanges(server.ecs.init_client()).construct()?.send(&server.handler, requester_info.endpoint);

        // spawns player
        spawn_player(&mut server.ecs, requester_id);

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

    } else {
        println!("Participant with name '{requester_id}' already exists");
    }

    Ok(())
}
