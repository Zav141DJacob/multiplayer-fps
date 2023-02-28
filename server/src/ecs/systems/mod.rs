use common::Direction;

use crate::ecs::ServerEcs;

mod physics;

/// Server-side systems are implemented onto this
pub struct ServerSystems;

impl ServerSystems {
    pub fn run(ecs: &mut ServerEcs, dt: f32) {

        let players = ServerSystems::get_input_states(ecs);
        // dbg!(k, v);

        for (k, v) in players {
            dbg!(k, v);
            if v.forward {
                ServerSystems::apply_velocity(ecs, Direction::Forward, k);
            }
            
            if v.backward {
                ServerSystems::apply_velocity(ecs, Direction::Backward, k);
            }
            
            if v.left {
                ServerSystems::apply_velocity(ecs, Direction::Left, k);
            }
            
            if v.right {
                ServerSystems::apply_velocity(ecs, Direction::Right, k);
            }

            if v.look_left {
                ServerSystems::apply_turning(ecs, Direction::Left, k);
            }

            if v.look_right {
                ServerSystems::apply_turning(ecs, Direction::Right, k);
            }
            
            if v.shoot {
                ServerSystems::apply_shoot(ecs, k);
            }
        }
    }

    // let (entity, (_, look_direction, position, gun)) = self
    //         .ecs
    //         .world
    //         .query_mut::<(&Player, &mut LookDirection, &mut Position, &mut CurrentGun )>()
    //         .into_iter()
    //         .find(|(_, (&player, _, _, _))| player.id == name)
    //         .unwrap();

    
}
