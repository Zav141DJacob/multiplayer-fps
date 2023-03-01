use std::net::{IpAddr, SocketAddr};
use anyhow::anyhow;
use message_io::network::RemoteAddr;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::mpsc::error::TryRecvError;
use common::{FromClientMessage, FromServerMessage};
use crate::client::Client;

pub type ClientReceiver = UnboundedReceiver<FromServerMessage>;
pub type ClientSender = UnboundedSender<FromClientMessage>;

pub struct Connection {
    client: Client,
    receiver: ClientReceiver,
    sender: ClientSender,
}

impl Connection {
    pub fn new(ip: IpAddr, port: u16) -> anyhow::Result<Self> {
        let addr = RemoteAddr::Socket(SocketAddr::new(ip, port));
        let mut client = Client::new(addr)?;
        let (receiver, sender) = client.start()?;

        Ok(Self {
            client,
            receiver,
            sender,
        })
    }

    pub fn receive(&mut self) -> anyhow::Result<Option<FromServerMessage>> {
        let res = self.receiver.try_recv();
        match res {
            Ok(message) => Ok(Some(message)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(anyhow!("Client disconnected from server"))
        }
    }

    pub fn send(&mut self, message: FromClientMessage) -> anyhow::Result<()> {
        self.sender.send(message).map_err(|e| e.into())
    }
}
