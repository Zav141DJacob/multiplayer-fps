use std::net::IpAddr;

use clap::Parser;
use once_cell::sync::Lazy;

use common::defaults::{IP, PORT};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Port to connect to server on
    #[arg(short, long, default_value_t = PORT)]
    pub port: u16,

    /// IP to connect to server on
    #[arg(short, long, default_value_t = IP)]
    pub ip: IpAddr,
}

pub static ARGS: Lazy<Args> = Lazy::new(Args::parse);
