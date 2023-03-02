use std::{net::SocketAddr, time::Duration};

use common::{FromClientMessage, FromServerMessage};
use message_io::{
    network::{Endpoint, NetEvent, RemoteAddr, Transport},
    node::{self, NodeEvent, NodeHandler, NodeListener},
};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

enum Signal {
    Ping, // TODO: remove ping in the future
    Stop,
}

pub struct Client {
    handler: NodeHandler<Signal>,
    listener: Option<NodeListener<Signal>>,

    server_id: Endpoint,
    local_addr: SocketAddr,
}

impl Drop for Client {
    fn drop(&mut self) {
        self.stop();
    }
}

impl Client {
    pub fn new(remote_addr: RemoteAddr) -> anyhow::Result<Self> {
        let (handler, listener) = node::split();

        let (server_id, local_addr) = handler.network().connect(Transport::Udp, remote_addr)?;

        Ok(Client {
            handler,
            listener: Some(listener),
            server_id,
            local_addr,
        })
    }

    pub fn stop(&mut self) {
        self.handler.signals().send(Signal::Stop);
    }

    pub fn start(
        &mut self,
    ) -> anyhow::Result<(
        UnboundedReceiver<FromServerMessage>,
        UnboundedSender<FromClientMessage>,
    )> {
        // Messages recieved
        let (from_server_sender, from_server_reciever) =
            mpsc::unbounded_channel::<FromServerMessage>();
        // Messages sent
        let (from_client_sender, mut from_client_reciever) =
            mpsc::unbounded_channel::<FromClientMessage>();

        // Sends join event
        from_client_sender.send(FromClientMessage::Join)?;

        // Handles sent messages
        let handler = self.handler.clone();
        let server_id = self.server_id;

        tokio::spawn(async move {
            while let Some(message) = from_client_reciever.recv().await {
                let output_data = bincode::serialize(&message).unwrap();
                handler.network().send(server_id, &output_data);

                if let FromClientMessage::Leave = message {
                    break;
                }
            }

            from_client_reciever.close();
        });

        // Handles recieved messages
        let listener = self.listener.take().unwrap();
        let handler = self.handler.clone();
        let local_addr = self.local_addr;

        let from_client_sender2 = from_client_sender.clone();
        tokio::spawn(async move {
            listener.for_each(move |event| match event {
                NodeEvent::Network(net_event) => match net_event {
                    NetEvent::Connected(_, established) => {
                        if established {
                            println!("Connected to server at {}", server_id.addr(),);
                            println!("Client identified by local port: {}", local_addr.port());
                            handler.signals().send(Signal::Ping);
                        } else {
                            println!("Cant connect to server")
                        }
                    }
                    NetEvent::Accepted(_, _) => unreachable!(), // Only generated when a listener accepts
                    NetEvent::Message(_, input_data) => {
                        from_server_sender
                                .send(bincode::deserialize(input_data).unwrap()).expect("Failed to send message from server to client, this should never happen as messages shouldn't be sent here when listener has been closed")
                    }
                    NetEvent::Disconnected(_) => {
                        // TODO: Im pretty sure this never gets called, please look into it
                        println!("Server disconnected");
                        handler.stop();
                    }
                },
                NodeEvent::Signal(signal) => match signal {
                    Signal::Ping => { // TODO: remove ping in the future
                        if from_client_sender2.is_closed() {
                            return
                        }

                        let message = FromClientMessage::Ping;
                        let output_data = bincode::serialize(&message).unwrap();
                        handler.network().send(server_id, &output_data);
                        handler
                            .signals()
                            .send_with_timer(Signal::Ping, Duration::from_secs(1));
                    }
                    Signal::Stop => {
                        // Should never give an error but if it does it doesn't matter as its part of close process anyway
                        let _ = from_client_sender2.send(FromClientMessage::Leave);
                        handler.stop();
                    }
                },
            });
        });

        Ok((from_server_reciever, from_client_sender))
    }
}
