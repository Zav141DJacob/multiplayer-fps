use crate::ecs::systems::ServerSystems;
use crate::ecs::ServerEcs;
use common::ecs::components::{ HeldWeapon, WeaponCrate, Position };

impl ServerSystems {
    pub fn pick_up_system(ecs: &mut ServerEcs, _dt: f32) {
        let player_query = ecs
            .world
            .query_mut::<(&Position, &HeldWeapon)>()
            .into_iter()
            .map(|(e, (&p, &w))| (e, p, w))
            .collect::<Vec<_>>();

        let crate_query = ecs
            .world
            .query_mut::<(&Position, &WeaponCrate)>()
            .into_iter()
            .map(|(e, (&p, &c))| (e, p, c))
            .collect::<Vec<_>>();

        for p in player_query {
            for c in &crate_query {
                if (p.1.0.x - c.1.0.x).abs() < 0.3 && (p.1.0.y - c.1.0.y).abs() < 0.3 {
                    let e = ecs.world.entity(p.0).unwrap();
                    e.get::<&mut HeldWeapon>().unwrap().0 = c.2.0;
                    e.get::<&mut HeldWeapon>().unwrap().0.recharge();
                    ecs.observed_world().despawn(c.0).unwrap();
                    println!("a weapon was picked up");
                }
            }
        }
    }
}
