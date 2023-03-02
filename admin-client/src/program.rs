use std::net::{IpAddr, SocketAddr};

use notan::{
    egui::{self, EguiPluginSugar},
    prelude::{App, Assets, Color, Graphics, Plugins},
    AppState,
};
use server::server::Server;
use tokio::{sync::mpsc::UnboundedReceiver, task::JoinHandle};

#[derive(AppState)]
pub struct Program {
    logger_reciever: UnboundedReceiver<String>,
    server_join_handle: JoinHandle<()>,

    ip: IpAddr,
    port: u16,
    messages: Vec<String>,
}

pub fn notan_setup(
    ip: IpAddr,
    port: u16,
) -> Box<dyn Fn(&mut App, &mut Assets, &mut Graphics, &mut Plugins) -> Program> {
    Box::new(move |_, _, _, _| {
        let addr = SocketAddr::new(ip, port);
        println!("Starting server on {addr}");
        let (mut server, logger_reciever) = Server::new(addr, true).unwrap();

        let server_join_handle = tokio::spawn(async move {
            server.run();
        });

        Program {
            logger_reciever,
            server_join_handle,
            ip,
            port,
            messages: Vec::new(),
        }
    })
}

impl Program {
    pub fn notan_draw(
        _app: &mut App,
        _assets: &mut Assets,
        gfx: &mut Graphics,
        plugins: &mut Plugins,
        this: &mut Self,
    ) {
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

                    ui.label(format!("Server running on {}:{}", this.ip, this.port));

                    if ui.button("Stop server").clicked() {
                        this.server_join_handle.abort()
                    }
                });

                for msg in this.messages.clone() {
                    ui.label(msg);
                }
            });
        });

        output.clear_color(Color::BLACK);

        if output.needs_repaint() {
            gfx.render(&output);
        }
    }
}
