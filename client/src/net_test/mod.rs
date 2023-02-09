
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::net::{IpAddr, SocketAddr};


use itertools::Itertools;
use message_io::network::RemoteAddr;
use notan::app::{App, Graphics, Plugins};
use notan::egui::{self, ComboBox, EguiPluginSugar, ScrollArea, TextEdit, Ui};
use notan::prelude::{Assets, Color};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::mpsc::error::TryRecvError;
use tracing::error;

use common::{Direction, FromClientMessage, FromServerMessage};

use crate::args::ARGS;
use crate::client::Client;

use crate::program::state::ProgramState;

pub struct NetworkTest {
    ip: IpAddr,
    port: u16,
    connection: Option<Connection>,
    log: VecDeque<String>,
}

impl Display for NetworkTest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Network Test")
    }
}

impl NetworkTest {
    pub fn new() -> Self {
        Self {
            ip: ARGS.ip,
            port: ARGS.port,
            connection: None,
            log: VecDeque::with_capacity(1010),
        }
    }
}

struct Connection {
    receiver: UnboundedReceiver<FromServerMessage>,
    sender_widget: SenderWidget,
}

impl Connection {
    fn new(_ip: IpAddr, _port: u16) -> anyhow::Result<Self> {
        let addr = RemoteAddr::Socket(SocketAddr::new(ARGS.ip, ARGS.port));
        let mut client = Client::new(addr)?;
        let (receiver, sender) = client.start()?;

        Ok(Self {
            sender_widget: SenderWidget::new(sender.clone()),
            receiver,
        })
    }
}

impl ProgramState for NetworkTest {
    fn draw(&mut self, _app: &mut App, _assets: &mut Assets, gfx: &mut Graphics, plugins: &mut Plugins) {
        let mut output = plugins.egui(|ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let text = self.log.iter().rev().join("\n");
                let mut text = text.as_str();

                ScrollArea::vertical().stick_to_bottom(true).auto_shrink([false; 2]).show(ui, |ui| {
                    TextEdit::multiline(&mut text).show(ui);
                })
            });

            if self.connection.is_none() {
                egui::Window::new("Connect").show(ctx, |ui| {
                    ui.label(format!("IP: {}:{}", self.ip, self.port));
                    if ui.button("CONNECT").clicked() {
                        let conn = match Connection::new(self.ip, self.port) {
                            Ok(v) => v,
                            Err(err) => {
                                error!("Error connecting to server: {}", err);
                                return;
                            }
                        };
                        self.connection = Some(conn);
                    }
                });
                return;
            }

            let Connection {
                receiver,
                sender_widget,
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
    }
}


struct SenderWidget {
    sender: UnboundedSender<FromClientMessage>,
    mov_dir: Direction,
}

impl SenderWidget {
    fn new(sender: UnboundedSender<FromClientMessage>) -> Self {
        Self {
            sender,
            mov_dir: Direction::Forward,
        }
    }

    fn show(&mut self, ui: &mut Ui) -> anyhow::Result<()> {
        // If you are here due to a compile error after changing FromClientMessage,
        // then just comment out the appropriate section below.

        if ui.button("Ping").clicked() {
            self.sender.send(FromClientMessage::Ping)?
        }

        if ui.button("Join").clicked() {
            self.sender.send(FromClientMessage::Join)?
        }

        if ui.button("Leave").clicked() {
            self.sender.send(FromClientMessage::Leave)?
        }

        ui.separator();
        let direction = &mut self.mov_dir;
        ComboBox::from_label("Direction")
            .selected_text(format!("{direction:?}"))
            .show_ui(ui, |ui| {
                ui.selectable_value(direction, Direction::Forward, "Forward");
                ui.selectable_value(direction, Direction::Backward, "Backward");
                ui.selectable_value(direction, Direction::Left, "Left");
                ui.selectable_value(direction, Direction::Right, "Right");
            });
        if ui.button("Move").clicked() {
            self.sender.send(FromClientMessage::Move(*direction))?
        }

        Ok(())
    }
}
