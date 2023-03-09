// Components shared between client and server go here

use std::num::NonZeroU64;

use glam::Vec2;
use serde::{Deserialize, Serialize};
use crate::gun::Gun;

use crate::{register_shared_components, bulk_attribute, UserID};

/// Describes the instructions the server can give to the client ECS.
/// The contained NonZeroU64 is the entity ID.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EcsProtocol {
    Insert((NonZeroU64, InsertComponent)),
    Remove((NonZeroU64, RemoveComponent)),
    Despawn(NonZeroU64),
}

// Create InsertComponent and RemoveComponent enums with this macro
// This is only necessary for the components that get sent over the network
register_shared_components! {
    Position,
    Velocity,
    LookDirection,
    Size,
    Health,
    HeldWeapon,
    Player,
    Bullet,
    WeaponCrate,
}

// This macro simply adds derives for all these structs.
// You can just as easily define structs outside it and derive stuff manually
bulk_attribute! {
    derive(Debug, Clone, Copy, Serialize, Deserialize);
    pub struct Position (pub Vec2);
    pub struct Velocity (pub Vec2);
    pub struct LookDirection (pub Vec2);
    pub struct Size (pub Vec2);
    pub struct Health (pub i32);
    pub struct HeldWeapon (pub Gun);
    
    pub struct Player {
        pub id: UserID,
    }

    pub struct Bullet;
    pub struct WeaponCrate (pub Gun);
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct InputState {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,

    pub look_angle: f32,
    
    pub shoot: bool,
} 
