use std::io;
use std::net::IpAddr;

use common::defaults::IP;
use common::defaults::PORT;

use clap::Parser;
use server::run_server;

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

    run_server(args.ip, args.port)?;

    Ok(())
}
