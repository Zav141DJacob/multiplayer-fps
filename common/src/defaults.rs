use std::net::{IpAddr, Ipv4Addr};

pub const GAME_NAME: &str = "Multiplayer FPS";

pub const IP: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
pub const PORT: u16 = 1337;

pub const MAP_WIDTH: usize = 21; // Must be odd
pub const MAP_HEIGHT: usize = 21; // Must be odd
pub const MAP_BRANCHING: f32 = 0.5; // 0.0..=1.0
pub const MAP_OPENNESS: f32 = 0.2; // 0.0..=1.0

pub const PLAYER_MAX_HP: u32 = 100;
pub const DEFAULT_PLAYER_HP: u32 = PLAYER_MAX_HP;
pub const DEFAULT_PLAYER_NAME: &str = "Player";
pub const PLAYER_SPEED: f32 = 0.1;
pub const PLAYER_SIZE: f32 = 0.25;
pub const WEAPON_CRATES_AMOUNT: u32 = 5;

pub const TICKS_PER_SECOND: u64 = 144;
