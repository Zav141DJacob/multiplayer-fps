use hecs::World;
use resources::Resources;

use crate::ecs::observer::{ObservedWorld, Observer};
use crate::ecs::systems::ServerSystems;

mod observer;
mod components;
mod systems;

#[derive(Default)]
pub struct ServerEcs {
    world: World,
    observer: Observer,
    resources: Resources,
}

impl ServerEcs {
    /// Convenience function to get an [ObservedWorld]
    pub fn observed_world(&mut self) -> ObservedWorld {
        self.observer.observe(&mut self.world)
    }

    /// Runs the ECS systems with a given time since last tick in seconds (i.e. delta time)
    pub fn tick(&mut self, dt: f32) {
        ServerSystems::run(self, dt);
    }
}