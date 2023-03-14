use crate::ecs::systems::ServerSystems;
use crate::ecs::ServerEcs;
use common::ecs::components::{ Position, Health };
use common::map::Map;

impl ServerSystems {
    pub fn respawn_system(ecs: &mut ServerEcs, _dt: f32) {
        let player_query = ecs
            .world
            .query_mut::<(&mut Position, &Health)>();

        for (e, (p, h)) in player_query {
            if h.0 == 0 {
                let mut p = ecs.observer.observe_component(e, p);
                *p = ecs.resources.get::<Map>().unwrap().random_empty_spot().unwrap();
            }
        }
    }
}
