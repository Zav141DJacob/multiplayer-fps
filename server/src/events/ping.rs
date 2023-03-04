use common::{FromServerMessage, Signal};
use message_io::{network::Endpoint, node::NodeHandler};
use crate::constructed_message::ConstructMessage;

use crate::server::Logger;

pub fn execute(
    logger: &Logger,
    handler: &NodeHandler<Signal>,
    endpoint: Endpoint,
) -> Result<(), bincode::Error> {
    logger.log(format!("Ping from {}", endpoint.addr()));

    FromServerMessage::Pong.construct()?.send(handler, endpoint);

    Ok(())
}
