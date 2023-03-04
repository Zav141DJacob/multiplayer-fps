use std::{
    io,
    net::{IpAddr, SocketAddr},
};

use crate::server::Server;

pub mod ecs;
pub mod events;
pub mod server;
mod constructed_message;


pub fn run_server(ip: IpAddr, port: u16) -> io::Result<()> {
    let addr = SocketAddr::new(ip, port);
    println!("Starting server on {addr}");
    let (mut server, _) = Server::new(addr, false)?;
    server.run();

    Ok(())
}
