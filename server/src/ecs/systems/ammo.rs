use crate::ecs::ServerEcs;
use crate::{ecs::systems::ServerSystems};
use common::ecs::components::HeldWeapon;
use common::gun::Gun;

impl ServerSystems {
    /// Move all entities with a position and velocity
    pub fn reset_to_pistol(ecs: &mut ServerEcs, _dt: f32) {
        let query = ecs.world.query_mut::<&mut HeldWeapon>();

        for (_, weapon) in query {
            if weapon.ammo == 0 {
                weapon.gun = Gun::Pistol;
                weapon.ammo = weapon.gun.max_ammo();
            }
        }
    }
}
