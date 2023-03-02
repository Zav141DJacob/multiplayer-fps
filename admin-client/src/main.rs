use std::net::IpAddr;

use admin_client::run_admin_client;
use clap::Parser;
use common::defaults::{IP, PORT};

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
async fn main() -> Result<(), String> {
    let args = Args::parse();

    run_admin_client(args.ip, args.port)?;

    Ok(())
}
