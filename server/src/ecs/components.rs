// Server-only components go here

use common::UserID;

#[derive(Debug, Clone, Copy)]
pub struct Speed (pub f32);

#[derive(Debug, Clone, Copy)]
pub struct ShotBy {
    pub id: Option<UserID>
}

// Timer specifiers
pub struct ShootCooldown;
pub struct BulletDespawn;
