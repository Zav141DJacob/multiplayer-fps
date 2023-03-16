use clap::Parser;
use once_cell::sync::Lazy;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// If to enable debug features
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,
}

pub static ARGS: Lazy<Args> = Lazy::new(Args::parse);
