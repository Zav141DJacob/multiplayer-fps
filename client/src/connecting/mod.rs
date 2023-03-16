use admin_client::program::Program;
use anyhow::bail;
use std::fmt::{Display, Formatter};
use std::net::IpAddr;
use std::time::Instant;

use crate::game::ecs::ClientEcs;
use crate::game::net::Connection;
use crate::game::Game;
use crate::menu::Menu;
use common::ecs::components::Player;
use common::map::Map;
use common::{FromClientMessage, FromServerMessage};
use notan::app::{App, Graphics, Plugins};
use notan::egui::{self, Align2, EguiPluginSugar};
use notan::prelude::{Assets, Color};
use tracing::info;

use crate::program::state::ProgramState;

pub struct Connecting {
    start_time: Instant,
    connection: Option<Connection>,
    ecs: Option<ClientEcs>,
    my_id: Option<u64>,

    exit: bool,
    recieved_own_id: bool,

    username: String,
}

impl Display for Connecting {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Connecting")
    }
}

impl Connecting {
    pub fn new(
        address: IpAddr,
        port: u16,
        server_ui: Option<Program>,
        username: &str,
    ) -> anyhow::Result<Self> {
        let connection = Connection::new(address, port, username)?;
        let mut ecs = ClientEcs::default();

        if let Some(su) = server_ui {
            ecs.resources.insert(su);
        }

        Ok(Self {
            start_time: Instant::now(),
            connection: Some(connection),
            ecs: Some(ecs),
            my_id: None,

            exit: false,
            recieved_own_id: false,

            username: username.to_string(),
        })
    }
}

impl ProgramState for Connecting {
    fn update(
        &mut self,
        _app: &mut App,
        _assets: &mut Assets,
        _plugins: &mut Plugins,
    ) -> anyhow::Result<()> {
        let connection = self.connection.as_mut().unwrap();
        let message = match connection.receive() {
            Ok(Some(message)) => message,
            Ok(None) => return Ok(()),
            Err(err) => {
                bail!(err);
            }
        };

        match message {
            FromServerMessage::OwnId(my_id) => {
                self.recieved_own_id = true;
                info!("Received OwnId");
                self.my_id = Some(my_id);
            }
            FromServerMessage::SendMap(map) => {
                info!("Received SendMap");
                self.ecs.as_mut().unwrap().resources.insert(map);
            }
            FromServerMessage::Pong => {
                info!("Pong");

                if !self.recieved_own_id {
                    connection.send(FromClientMessage::Join(self.username.to_string()))?;
                }
            }
            FromServerMessage::EcsChanges(changes) => {
                info!("Received EcsChanges");
                for change in changes {
                    self.ecs.as_mut().unwrap().handle_protocol(change)?;
                }
            }
        }

        Ok(())
    }

    fn draw(
        &mut self,
        _app: &mut App,
        _assets: &mut Assets,
        gfx: &mut Graphics,
        plugins: &mut Plugins,
    ) -> anyhow::Result<()> {
        let mut err = None;

        let mut output = plugins.egui(|ctx| {
            let window = egui::Window::new("Connecting")
                .resizable(false)
                .collapsible(false)
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0]);

            window.show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    if ui.button("Ping").clicked() {
                        err = self
                            .connection
                            .as_mut()
                            .unwrap()
                            .send(FromClientMessage::Ping)
                            .err();
                    }

                    if ui.button("Return").clicked() {
                        self.exit = true;
                    }
                })
            });
        });

        output.clear_color(Color::BLACK);

        if output.needs_repaint() {
            gfx.render(&output);
        }

        Ok(())
    }

    fn change_state(
        &mut self,
        app: &mut App,
        _assets: &mut Assets,
        gfx: &mut Graphics,
        _plugins: &mut Plugins,
    ) -> Option<Box<dyn ProgramState>> {
        if self.exit {
            return Some(Menu::default().into());
        }

        // Make sure we've received the map
        let map_exists = self.ecs.as_mut()?.resources.get::<Map>().is_ok();
        if !map_exists {
            return None;
        }

        // Try to find the our player from the ecs
        let my_player_entity = self.my_id.and_then(|id| {
            for (entity, player) in self.ecs.as_mut()?.world.query_mut::<&Player>() {
                if player.id == id {
                    return Some(entity);
                }
            }
            None
        })?;

        // TODO: Add a message that shows the server has finished initializing us

        let ecs = self.ecs.take()?;
        let connection = self.connection.take()?;
        let game = Game::new(app, gfx, ecs, connection, my_player_entity);

        Some(game.into())
    }
}
