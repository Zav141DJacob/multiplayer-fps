use crate::client::{Client, ClientError};
use anyhow::anyhow;
use common::{FromClientMessage, FromServerMessage};
use message_io::network::RemoteAddr;
use std::net::{IpAddr, SocketAddr};
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub type ClientReceiver = UnboundedReceiver<Result<FromServerMessage, ClientError>>;
pub type ClientSender = UnboundedSender<FromClientMessage>;

pub struct Connection {
    client: Client,
    receiver: ClientReceiver,
    sender: ClientSender,
}

impl Connection {
    pub fn new(ip: IpAddr, port: u16, username: &str) -> anyhow::Result<Self> {
        let addr = RemoteAddr::Socket(SocketAddr::new(ip, port));
        let mut client = Client::new(addr)?;
        let (receiver, sender) = client.start(username)?;

        Ok(Self {
            client,
            receiver,
            sender,
        })
    }

    pub fn receive(&mut self) -> anyhow::Result<Option<FromServerMessage>> {
        match self.receiver.try_recv() {
            Ok(Ok(message)) => Ok(Some(message)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) | Ok(Err(ClientError::Disconnected)) => {
                Err(anyhow!(ClientError::Disconnected))
            }
        }
    }

    pub fn send(&mut self, message: FromClientMessage) -> anyhow::Result<()> {
        self.sender.send(message).map_err(|e| e.into())
    }
}
