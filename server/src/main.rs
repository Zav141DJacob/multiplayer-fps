use std::io;
use std::net::{IpAddr, SocketAddr};

use common::defaults::IP;
use common::defaults::PORT;

use clap::Parser;
use server::server::Server;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to host server on
    #[arg(short, long, default_value_t = PORT)]
    port: u16,

    /// IP to host server on
    #[arg(short, long, default_value_t = IP)]
    ip: IpAddr,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();
    let addr: SocketAddr = SocketAddr::new(args.ip, args.port);

    println!("Starting server on {addr}");
    let mut server = Server::new(addr)?;
    server.run();

    Ok(())
}
