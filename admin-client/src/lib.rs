use std::net::IpAddr;

use notan::{draw::DrawConfig, egui::EguiConfig, prelude::WindowConfig};
use program::{notan_draw, notan_setup};

pub mod program;

pub fn run_admin_client(ip: IpAddr, port: u16) -> Result<(), String> {
    let win = WindowConfig::new()
        .vsync(true)
        .high_dpi(false)
        .resizable(false)
        .size(640, 360);

    notan::init_with(notan_setup(ip, port, false))
        .add_config(win)
        .add_config(EguiConfig)
        .add_config(DrawConfig)
        .draw(notan_draw)
        .build()
}
