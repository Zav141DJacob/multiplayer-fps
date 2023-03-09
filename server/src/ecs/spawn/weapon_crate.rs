use common::ecs::components::WeaponCrate;
use common::gun::Gun;
use common::map::Map;
use hecs::Entity;

use crate::ecs::ServerEcs;

pub fn spawn_weapon_crate(ecs: &mut ServerEcs) -> Entity {
    let entity = ecs.world.reserve_entity();
    let pos = ecs.resources.get::<Map>().unwrap().random_empty_spot();

    ecs.observed_world().insert(entity, (
        WeaponCrate(Gun::get_random_gun()),
        pos,
    )).unwrap();

    entity
}