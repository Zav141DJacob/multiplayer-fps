use std::num::NonZeroU64;

use anyhow::Result;
use hecs::Entity;

use common::ecs::components::EcsProtocol;

use crate::game::ecs::ClientEcs;

impl ClientEcs {
    /// Retrieves a mapped entity or reserves a new one
    fn map_entity(&mut self, entity_id: NonZeroU64) -> Entity {
        *self.entity_map.entry(entity_id)
            .or_insert_with_key(|_| self.world.reserve_entity())
    }

    /// Handle an [EcsProtocol] message from the server
    pub fn handle_protocol(&mut self, message: EcsProtocol) -> Result<()> {
        match message {
            EcsProtocol::Insert((entity_id, insert)) => {
                let entity = self.map_entity(entity_id);
                insert.apply(&mut self.world, entity)?;
            }
            EcsProtocol::Remove((entity_id, remove)) => {
                let entity = self.map_entity(entity_id);
                remove.apply(&mut self.world, entity)?;
            }
            EcsProtocol::Despawn(entity_id) => {
                if let Some(entity) = self.entity_map.remove(&entity_id) {
                    self.world.despawn(entity)?;
                }
            }
        }
        Ok(())
    }
}
