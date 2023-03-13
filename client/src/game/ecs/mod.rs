use std::collections::HashMap;
use std::num::NonZeroU64;
use hecs::{Entity, World};
use resources::Resources;
use crate::game::ecs::systems::ClientSystems;

pub mod net;
pub mod systems;
pub mod component;

#[derive(Default)]
pub struct ClientEcs {
    pub world: World,
    pub resources: Resources,

    /// Keeps track of what server entity maps to what client entity
    entity_map: HashMap<NonZeroU64, Entity>,
}

impl ClientEcs {
    /// Runs the ECS systems with a given time since last tick in seconds (i.e. delta time)
    pub fn tick(&mut self, dt: f32) {
        ClientSystems::run(self, dt);
    }
}

pub struct MyEntity(pub Entity);