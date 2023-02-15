use std::net::{IpAddr, Ipv4Addr};

pub const IP: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
pub const PORT: u16 = 1337;

pub const MAP_WIDTH: usize = 10;
pub const MAP_HEIGHT: usize = 10;