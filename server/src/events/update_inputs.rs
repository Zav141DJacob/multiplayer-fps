use std::{error::Error, fmt::Display};

use common::ecs::components::InputState;
use hecs::QueryOneError;
use message_io::network::Endpoint;

use crate::server::Server;

#[derive(Debug)]
pub enum InputError {
    FailedToGetPlayer,
    FailedToFindPlayerInEcs(QueryOneError),
}

impl From<QueryOneError> for InputError {
    fn from(value: QueryOneError) -> Self {
        InputError::FailedToFindPlayerInEcs(value)
    }
}

impl Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputError::FailedToGetPlayer => {
                write!(f, "tried to update input for unregistered client")
            }
            InputError::FailedToFindPlayerInEcs(e) => {
                write!(f, "client is registered but not found in ecs: {e}")
            }
        }
    }
}

impl Error for InputError {}

pub fn execute(
    server: &mut Server,
    updated_input_state: InputState,
    endpoint: Endpoint,
) -> Result<(), InputError> {
    let entity = *server
        .registered_clients
        .get(&endpoint)
        .ok_or(InputError::FailedToGetPlayer)?;

    let input = server.ecs.world.query_one_mut::<&mut InputState>(entity)?;

    *input = updated_input_state;

    Ok(())
}
