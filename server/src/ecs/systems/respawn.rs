use crate::ecs::systems::ServerSystems;
use crate::ecs::ServerEcs;
use common::defaults::DEFAULT_PLAYER_HP;
use common::ecs::components::{
    DeadPlayer, Deaths, Health, HeldWeapon, Kills, Player, Position, ShotBy,
};
use common::ecs::timer::Timer;
use common::map::Map;
use std::time::Duration;

impl ServerSystems {
    pub fn respawn_system(ecs: &mut ServerEcs, _dt: f32) {
        let player_query = ecs.world.query_mut::<(
            &mut Position,
            &mut Health,
            &mut HeldWeapon,
            &mut Deaths,
            &ShotBy,
        )>();
        let mut killers: Vec<Option<u64>> = Vec::new();
        let mut death_positions = vec![];

        for (e, (p, h, w, d, s_b)) in player_query {
            if h.0 == 0 {
                death_positions.push(p.0);

                let mut p = ecs.observer.observe_component(e, p);
                *p = ecs
                    .resources
                    .get::<Map>()
                    .unwrap()
                    .random_empty_spot()
                    .expect("Can't find a random spot");
                drop(p);

                let mut h = ecs.observer.observe_component(e, h);
                h.0 = DEFAULT_PLAYER_HP;
                drop(h);

                let mut w = ecs.observer.observe_component(e, w);
                w.ammo = w.gun.get_max_ammo();
                drop(w);

                let mut d = ecs.observer.observe_component(e, d);
                d.0 += 1;
                drop(d);

                killers.push(s_b.id)
            }
        }

        for (entity, (player, kills)) in ecs.world.query_mut::<(&Player, &mut Kills)>() {
            for killer in killers.iter().flatten() {
                if killer == &player.id {
                    let mut kills = ecs.observer.observe_component(entity, kills);
                    kills.0 += 1;
                    break;
                }
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
