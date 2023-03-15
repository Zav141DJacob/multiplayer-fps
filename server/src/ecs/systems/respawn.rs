use common::defaults::DEFAULT_PLAYER_HP;
use crate::ecs::systems::ServerSystems;
use crate::ecs::ServerEcs;
use common::ecs::components::{ Position, Health, HeldWeapon };
use common::map::Map;

impl ServerSystems {
    pub fn respawn_system(ecs: &mut ServerEcs, _dt: f32) {
        let player_query = ecs
            .world
            .query_mut::<(&mut Position, &mut Health, &mut HeldWeapon)>();

        for (e, (p, h, w)) in player_query {
            if h.0 == 0 {
                let mut p = ecs.observer.observe_component(e, p);
                *p = ecs.resources.get::<Map>().unwrap().random_empty_spot();
                drop(p);

                let mut h = ecs.observer.observe_component(e, h);
                h.0 = DEFAULT_PLAYER_HP;
                drop(h);

                let mut w = ecs.observer.observe_component(e, w);
                w.1 = w.0.get_max_ammo();
            }
        }
    }
}
