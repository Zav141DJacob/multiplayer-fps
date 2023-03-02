use std::net::{IpAddr, SocketAddr};

use message_io::node::NodeHandler;
use notan::{
    egui::{self, EguiPluginSugar, ScrollArea},
    prelude::{App, Assets, Color, Graphics, Plugins},
    AppState,
};
use server::server::Server;
use tokio::{sync::mpsc::UnboundedReceiver, task::JoinHandle};

#[derive(AppState)]
pub struct Program {
    logger_reciever: UnboundedReceiver<String>,
    server_handler: NodeHandler<()>,

    ip: IpAddr,
    port: u16,
    messages: Vec<String>,

    should_exit_on_server_closing: bool,
    server_open: bool,
}

pub fn notan_setup(
    ip: IpAddr,
    port: u16,
    should_exit_on_server_closing: bool,
) -> Box<dyn Fn(&mut App, &mut Assets, &mut Graphics, &mut Plugins) -> Program> {
    Box::new(move |_, _, _, _| {
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
            server_open: true,
        }
    })
}

impl Program {
    pub fn notan_draw(
        app: &mut App,
        _assets: &mut Assets,
        gfx: &mut Graphics,
        plugins: &mut Plugins,
        this: &mut Self,
    ) {
        if !this.server_handler.is_running() {
            this.server_open = false;
        }

        if !this.server_open && this.should_exit_on_server_closing {
            app.exit()
        }

        'msgloop: loop {
            match this.logger_reciever.try_recv() {
                Ok(msg) => this.messages.push(msg),
                Err(_) => break 'msgloop,
            }
        }

        let mut output = plugins.egui(|ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Admin console");
                    ui.add_space(10.0);

                    if this.server_open {
                        ui.label(format!("Server running on {}:{}", this.ip, this.port));

                        if ui.button("Stop server").clicked() {
                            this.server_handler.stop();
                            this.server_open = false;
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
                        for msg in this.messages.clone() {
                            ui.label(msg);
                        }
                    });
            });
        });

        output.clear_color(Color::BLACK);

        if output.needs_repaint() {
            gfx.render(&output);
        }
    }
}
