use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::net::{IpAddr, SocketAddr};

use itertools::Itertools;
use message_io::network::RemoteAddr;
use notan::egui::{self, EguiPluginSugar, ScrollArea, TextEdit, Ui};
use notan::prelude::{App, Assets, Color, Graphics, Plugins};
use std::net::ToSocketAddrs;
use tokio::sync::mpsc::error::TryRecvError;
use tracing::error;

use common::FromClientMessage;

use crate::client::Client;
use crate::errorwindow::ErrorWindows;
use crate::game::net::{ClientReceiver, ClientSender};
use crate::program::state::ProgramState;
use common::defaults::{DEFAULT_PLAYER_NAME, IP, PORT};

pub struct NetworkTest {
    connection: Option<Connection>,
    log: VecDeque<String>,

    ip: String,
    processed_ip: SocketAddr,
    errors: ErrorWindows,
}

impl Display for NetworkTest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Network Test")
    }
}

impl NetworkTest {
    pub fn new() -> Self {
        let socketaddr = SocketAddr::new(IP, PORT);

        Self {
            errors: ErrorWindows::new(),
            processed_ip: socketaddr,
            ip: socketaddr.to_string(),
            connection: None,
            log: VecDeque::with_capacity(1010),
        }
    }

    pub fn process_inputs(&mut self) -> bool {
        self.processed_ip = match self.ip.to_socket_addrs() {
            Ok(mut ip) => ip.next().unwrap(),
            Err(error) => {
                self.errors.add_error(error.to_string());
                return false;
            }
        };

        true
    }
}

struct Connection {
    client: Client,
    receiver: ClientReceiver,
    sender_widget: SenderWidget,
}

impl Connection {
    fn new(ip: IpAddr, port: u16, username: &str) -> anyhow::Result<Self> {
        let addr = RemoteAddr::Socket(SocketAddr::new(ip, port));
        let mut client = Client::new(addr)?;
        let (receiver, sender) = client.start(username)?;

        Ok(Self {
            client,
            sender_widget: SenderWidget::new(sender),
            receiver,
        })
    }
}

impl ProgramState for NetworkTest {
    fn draw(
        &mut self,
        _app: &mut App,
        _assets: &mut Assets,
        gfx: &mut Graphics,
        plugins: &mut Plugins,
    ) -> anyhow::Result<()> {
        let mut output = plugins.egui(|ctx| {
            self.errors.draw_errors(ctx);

            egui::CentralPanel::default().show(ctx, |ui| {
                let text = self.log.iter().rev().join("\n");
                let mut text = text.as_str();

                ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        TextEdit::multiline(&mut text).show(ui);
                    })
            });

            egui::Window::new("Connection").show(ctx, |ui| {
                egui::TextEdit::singleline(&mut self.ip)
                    .hint_text("IP")
                    .show(ui);

                if self.connection.is_none() {
                    if ui.button("CONNECT").clicked() && self.process_inputs() {
                        let conn = match Connection::new(
                            self.processed_ip.ip(),
                            self.processed_ip.port(),
                            DEFAULT_PLAYER_NAME,
                        ) {
                            Ok(v) => v,
                            Err(err) => {
                                error!("Error connecting to server: {}", err);
                                return;
                            }
                        };
                        self.connection = Some(conn);
                    }
                } else if ui.button("DISCONNECT").clicked() {
                    self.connection = None;
                }
            });

            if self.connection.is_none() {
                return;
            }

            let Connection {
                receiver,
                sender_widget,
                ..
            } = self.connection.as_mut().unwrap();

            loop {
                // Using a manual match instead of while let to detect disconnects
                let message = match receiver.try_recv() {
                    Ok(v) => v,
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => {
                        self.connection = None;
                        error!("Disconnected from server");
                        return;
                    }
                };

                self.log.push_front(format!("{message:?}"));
            }

            egui::Window::new("Send").show(ctx, |ui| {
                if let Err(err) = sender_widget.show(ui) {
                    error!("Error sending message: {}", err)
                }
            });
        });

        output.clear_color(Color::BLACK);
        gfx.render(&output);

        Ok(())
    }
}

struct SenderWidget {
    sender: ClientSender,
    username: String,
}

impl SenderWidget {
    fn new(sender: ClientSender) -> Self {
        Self {
            sender,
            username: String::new(),
        }
    }

    fn show(&mut self, ui: &mut Ui) -> anyhow::Result<()> {
        // If you are here due to a compile error after changing FromClientMessage,
        // then just comment out the appropriate section below.

        if ui.button("Ping").clicked() {
            self.sender.send(FromClientMessage::Ping)?
        }

        egui::TextEdit::singleline(&mut self.username)
            .hint_text("Username")
            .show(ui);

        if ui.button("Join").clicked() {
            self.sender
                .send(FromClientMessage::Join(self.username.to_string()))?
        }

        if ui.button("Leave").clicked() {
            self.sender.send(FromClientMessage::Leave)?
        }

        // if ui.button("Move").clicked() {
        //     self.sender.send(FromClientMessage::Move(*direction))?
        // }

        Ok(())
    }
}
