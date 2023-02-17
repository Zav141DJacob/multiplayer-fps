use common::ecs::components::{EcsProtocol, InsertComponent};
use hecs::World;
use resources::Resources;

use crate::ecs::observer::{ObservedWorld, Observer};
use crate::ecs::systems::ServerSystems;

mod components;
mod observer;
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

    /// Get a sequence of [EcsProtocol] messages that will initialize a new client up to the current state of the server ECS
    pub fn init_client(&mut self) -> Vec<EcsProtocol> {
        InsertComponent::query_all(&mut self.world)
    }
}
