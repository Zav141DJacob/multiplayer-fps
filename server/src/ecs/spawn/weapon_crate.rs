use common::defaults::WEAPON_CRATES_AMOUNT;
use common::ecs::components::WeaponCrate;
use common::map::Map;
use hecs::Entity;

use crate::ecs::ServerEcs;

pub fn spawn_weapon_crate(ecs: &mut ServerEcs) -> Entity {
    let entity = ecs.world.reserve_entity();
    let pos = ecs
        .resources
        .get::<Map>()
        .unwrap()
        .random_empty_spot()
        .expect("Can't find a random spot");

    ecs.observed_world()
        .insert(entity, (WeaponCrate(rand::random()), pos))
        .unwrap();

    entity
}

pub fn spawn_weapon_crates_init(ecs: &mut ServerEcs) {
    for _ in 0..=WEAPON_CRATES_AMOUNT {
        spawn_weapon_crate(ecs);
    }
}
