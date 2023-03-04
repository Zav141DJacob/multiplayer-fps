use message_io::network::Endpoint;
use message_io::node::NodeHandler;
use common::{FromServerMessage, Signal};
use crate::server::RegisteredClients;

pub struct ConstructedMessage(Vec<u8>);

impl ConstructedMessage {
    pub fn send(&self, handler: &NodeHandler<Signal>, endpoint: Endpoint) {
        handler.network().send(endpoint, &self.0);
    }

    pub fn send_all(&self, handler: &NodeHandler<Signal>, registered_clients: &RegisteredClients) {
        for &endpoint in registered_clients.keys() {
            self.send(handler, endpoint)
        }
    }
}

pub trait ConstructMessage {
    fn construct(&self) -> Result<ConstructedMessage, bincode::Error>;
}

impl ConstructMessage for FromServerMessage {
    fn construct(&self) -> Result<ConstructedMessage, bincode::Error> {
        Ok(ConstructedMessage(bincode::serialize(self)?))
    }
}
