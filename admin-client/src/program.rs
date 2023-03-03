use std::net::{IpAddr, SocketAddr};

use common::Signal;
use message_io::node::NodeHandler;
use notan::{
    egui::{self, EguiPluginSugar, ScrollArea, Ui},
    prelude::{App, Assets, Color, Graphics, Plugins},
    AppState,
};
use server::server::Server;
use tokio::sync::mpsc::UnboundedReceiver;

#[derive(AppState)]
pub struct Program {
    logger_reciever: UnboundedReceiver<String>,
    server_handler: NodeHandler<Signal>,

    ip: IpAddr,
    port: u16,
    messages: Vec<String>,

    should_exit_on_server_closing: bool,
}

impl Program {
    pub fn new(ip: IpAddr, port: u16, should_exit_on_server_closing: bool) -> Program {
        let addr = SocketAddr::new(ip, port);
        println!("Starting server on {addr}");
        let (mut server, logger_reciever) = Server::new(addr, true).unwrap();
        let server_handler = server.handler.clone();
        tokio::spawn(async move {
            server.run();
        });

        Program {
            logger_reciever,
            ip,
            port,
            messages: Vec::new(),
            server_handler,
            should_exit_on_server_closing,
        }
    }

    pub fn draw(&mut self, ui: &mut Ui, app: &mut App) {
        if !self.server_handler.is_running() && self.should_exit_on_server_closing {
            app.exit()
        }

        'msgloop: loop {
            match self.logger_reciever.try_recv() {
                Ok(msg) => self.messages.push(msg),
                Err(_) => break 'msgloop,
            }
        }

        ui.vertical_centered(|ui| {
            ui.heading("Admin console");
            ui.add_space(10.0);

            if self.server_handler.is_running() {
                ui.label(format!("Server running on {}:{}", self.ip, self.port));

                if ui.button("Stop server").clicked() {
                    self.server_handler.stop();
                }
            } else {
                ui.label("Server is not currently running");
            }
        });

        ui.heading("Messages:");
        ScrollArea::vertical()
            .stick_to_bottom(true)
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for msg in self.messages.clone() {
                    ui.label(msg);
                }
            });
    }
}

pub fn notan_setup(
    ip: IpAddr,
    port: u16,
    should_exit_on_server_closing: bool,
) -> Box<dyn Fn(&mut App, &mut Assets, &mut Graphics, &mut Plugins) -> Program> {
    Box::new(move |_, _, _, _| Program::new(ip, port, should_exit_on_server_closing))
}

pub fn notan_draw(
    app: &mut App,
    _assets: &mut Assets,
    gfx: &mut Graphics,
    plugins: &mut Plugins,
    this: &mut Program,
) {
    let mut output = plugins.egui(|ctx| {
        egui::CentralPanel::default().show(ctx, |ui| this.draw(ui, app));
    });

    output.clear_color(Color::BLACK);

    if output.needs_repaint() {
        gfx.render(&output);
    }
}
