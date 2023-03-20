use std::net::{IpAddr, Ipv4Addr};
use crate::map::Textured;

pub const QUICK_JOIN_IP: &str = "fps.catnip.ee";

pub const GAME_NAME: &str = "Multiplayer FPS";

pub const IP: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
pub const PORT: u16 = 1337;

pub const MAP_WIDTH: usize = 13; // Must be odd
pub const MAP_HEIGHT: usize = MAP_WIDTH; // Must be odd
pub const MAP_BRANCHING: f32 = 0.5; // 0.0..=1.0
pub const MAP_OPENNESS: f32 = 0.3; // 0.0..=1.0

pub const MAP_DEFAULT_WALL: Textured = Textured::Brick2;
pub const MAP_SECTOR_COUNT: usize = 5;
pub const MAP_SECTOR_MIN_SIZE: usize = 4;
pub const MAP_SECTOR_MAX_SIZE: usize = 7;

pub const MINIMAP_SCALE: f32 = 2.0;

pub const PLAYER_MAX_HP: f32 = 100.0;
pub const DEFAULT_PLAYER_HP: f32 = PLAYER_MAX_HP;
pub const DEFAULT_PLAYER_NAME: &str = "Player";
pub const PLAYER_SPEED: f32 = 0.1;
pub const PLAYER_SIZE: f32 = 0.25;
pub const WEAPON_CRATES_AMOUNT: u32 = 5;

pub const TICKS_PER_SECOND: u64 = 144;
