use std::{
    io,
    net::{IpAddr, SocketAddr},
};

use crate::server::Server;

pub mod ecs;
pub mod events;
pub mod server;
pub mod utils;

pub fn run_server(ip: IpAddr, port: u16) -> io::Result<()> {
    let addr = SocketAddr::new(ip, port);
    println!("Starting server on {addr}");
    let mut server = Server::new(addr)?;
    server.run();

    Ok(())
}
