use std::time::Duration;
use common::defaults::DEFAULT_PLAYER_HP;
use crate::ecs::systems::ServerSystems;
use crate::ecs::ServerEcs;
use common::ecs::components::{Position, Health, HeldWeapon, DeadPlayer};
use common::ecs::timer::Timer;
use common::map::Map;

impl ServerSystems {
    pub fn respawn_system(ecs: &mut ServerEcs, _dt: f32) {
        let player_query = ecs
            .world
            .query_mut::<(&mut Position, &mut Health, &mut HeldWeapon)>();

        let mut death_positions = vec![];

        for (e, (p, h, w)) in player_query {
            if h.0 == 0 {
                death_positions.push(p.0);

                let mut p = ecs.observer.observe_component(e, p);
                *p = ecs.resources.get::<Map>().unwrap().random_empty_spot().expect("Can't find a random spot");
                drop(p);

                let mut h = ecs.observer.observe_component(e, h);
                h.0 = DEFAULT_PLAYER_HP;
                drop(h);

                let mut w = ecs.observer.observe_component(e, w);
                w.ammo = w.gun.get_max_ammo();
            }
        }

        // Dead player animations
        for pos in death_positions {
            let entity = ecs.observed_world().spawn((
                DeadPlayer,
                Position(pos),
            ));

            ecs.world.insert_one(
                entity,
                Timer::new(Duration::from_secs_f32(1.95), ())
            ).unwrap();
        }

        Timer::<()>::system_with(&mut ecs.world, |world, entity, _| {
            ecs.observer.observe(world).despawn(entity).unwrap();
        });
    }
}
