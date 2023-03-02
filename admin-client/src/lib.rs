use std::net::IpAddr;

use notan::{draw::DrawConfig, egui::EguiConfig, prelude::WindowConfig};
use program::notan_setup;

use crate::program::Program;

pub mod program;

use eframe::egui;

pub fn run_admin_client(ip: IpAddr, port: u16) -> Result<(), String> {
    // let win = WindowConfig::new()
    //     .vsync(true)
    //     .high_dpi(false)
    //     .resizable(false)
    //     .size(640, 360);

    // notan::init_with(notan_setup(ip, port, false))
    //     .add_config(win)
    //     .add_config(EguiConfig)
    //     .add_config(DrawConfig)
    //     .draw(Program::notan_draw)
    //     .build()
    let options = eframe::NativeOptions {
        run_and_return: true,
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    eframe::run_native(
        "First Window",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
    .unwrap();

    Ok(())
}

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Close").clicked() {
                eprintln!("Pressed Close button");
                frame.close();
            }
        });
    }
}
