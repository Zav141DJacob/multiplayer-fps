use std::{
    net::{IpAddr, SocketAddr},
    sync::{Arc, Mutex}, io,
};

use common::Signal;
use message_io::node::NodeHandler;
use notan::{
    egui::{self, EguiPluginSugar, ScrollArea, Ui},
    prelude::{App, Assets, Color, Graphics, Plugins},
    AppState,
};
use server::server::Server;

#[derive(AppState, Clone)]
pub struct Program {
    server_handler: Option<NodeHandler<Signal>>, // Contains handler when it has been ran

    ip: IpAddr,
    port: u16,
    messages: Arc<Mutex<Vec<String>>>,

    should_exit_on_server_closing: bool,
}

impl Program {
    pub fn new(ip: IpAddr, port: u16, should_exit_on_server_closing: bool) -> Program {
        Program {
            ip,
            port,
            messages: Arc::new(Mutex::new(Vec::new())),
            server_handler: None,
            should_exit_on_server_closing,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let addr = SocketAddr::new(self.ip, self.port);
        println!("Starting server on {addr}");
        let (mut server, mut logger_reciever) = Server::new(addr, true)?;
        self.server_handler = Some(server.handler.clone());

        tokio::spawn(async move {
            server.run();
        });

        let messages = Arc::clone(&self.messages);
        tokio::spawn(async move {
            loop {
                if let Ok(msg) = logger_reciever.try_recv() {
                    messages.lock().unwrap().push(msg)
                }
            }
        });

        Ok(())
    }

    pub fn is_running(&self) -> bool {
        match self.server_handler.clone() {
            Some(server_handler) => server_handler.is_running(),
            None => false,
        }
    }

    pub fn stop(&self) {
        if self.is_running() {
            self.server_handler.clone().unwrap().stop()
        }
    }

    pub fn draw(&mut self, ui: &mut Ui, app: &mut App) {
        if !self.is_running() && self.should_exit_on_server_closing {
            app.exit()
        }

        ui.vertical_centered(|ui| {
            ui.heading("Admin console");
            ui.add_space(10.0);

            if self.is_running() {
                ui.label(format!("Server running on {}:{}", self.ip, self.port));

                if ui.button("Stop server").clicked() {
                    self.stop();
                }
            } else {
                ui.label("Server is not currently running");
            }
        });

        ui.heading("Messages:");
        ui.add_space(5.0);
        ScrollArea::vertical()
            .stick_to_bottom(true)
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for msg in self.messages.lock().unwrap().iter() {
                    ui.label(msg);
                }
            });
    }
}

pub fn notan_setup(
    ip: IpAddr,
    port: u16,
    should_exit_on_server_closing: bool,
) -> impl Fn(&mut App, &mut Assets, &mut Graphics, &mut Plugins) -> Program {
    move |_, _, _, _| {
        let mut p = Program::new(ip, port, should_exit_on_server_closing);
        p.run().unwrap();
        p
    }
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
