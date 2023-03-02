use common::FromServerMessage;
use message_io::{network::Endpoint, node::NodeHandler};

use crate::server::Logger;

pub fn execute(
    logger: &Logger,
    handler: &NodeHandler<()>,
    endpoint: Endpoint,
) -> Result<(), bincode::Error> {
    logger.log(format!("Ping from {}", endpoint.addr()));

    FromServerMessage::Pong.construct()?.send(handler, endpoint);

    Ok(())
}
