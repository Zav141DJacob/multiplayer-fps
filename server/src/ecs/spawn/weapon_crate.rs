use common::ecs::components::WeaponCrate;
use common::gun::Gun;
use common::map::Map;
use hecs::Entity;

use crate::ecs::ServerEcs;

pub fn spawn_weapon_crate(ecs: &mut ServerEcs) -> Entity {
    let entity = ecs.world.reserve_entity();
    let pos = ecs.resources.get::<Map>().unwrap().random_empty_spot().unwrap();

    ecs.observed_world().insert(entity, (
        WeaponCrate(Gun::get_random_gun()),
        pos,
    )).unwrap();

    entity
}

pub fn spawn_weapon_crates_init(ecs: &mut ServerEcs) {
    for _ in 0..4 {
        spawn_weapon_crate(ecs);
    }
}
