use hecs::{Entity, World};
use rand::Rng;

use crate::{
    map::{Map, MapCell},
    Coordinates, Player, UserID,
};

pub type PlayerEntity = (Player, UserID, Coordinates);

pub fn spawn_player_at(coords: Coordinates, world: &mut World, player_id: u64) -> Entity {
    world.spawn((
        Player,
        player_id as UserID,
        Coordinates {
            x: coords.x,
            y: coords.y,
        },
    ))
}

pub fn spawn_player(map: &Map, world: &mut World, player: u64) -> (Coordinates, Entity) {
    let mut available_coords: Vec<Coordinates> = Vec::new();
    for x in 0..map.width {
        for y in 0..map.height {
            if map.cell(x, y) == MapCell::Empty {
                available_coords.push(Coordinates { x: x as f32, y: y as f32 });
            }
        }
    }

    let rand_num: usize = rand::thread_rng().gen_range(0..=available_coords.len());

    (
        available_coords[rand_num],
        spawn_player_at(available_coords[rand_num], world, player),
    )
}
