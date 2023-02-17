use std::{str::FromStr};
use serde::{Deserialize, Serialize};
use hecs::World;

use crate::{Player, UserID, Coordinates};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
    width: usize,
    height: usize,
    data: Vec<MapCell>,
}

impl Default for Map {
    fn default() -> Self {
        // Map::new(MAP_WIDTH, MAP_HEIGHT)
        let w = MapCell::Wall([1.0, 1.0, 1.0]);
        let e = MapCell::Empty;
        let temp_map = vec![
            w, w, w, w, w, w, w, w, w, w,
            w, e, w, e, e, e, e, e, e, w,
            w, e, w, e, e, e, e, e, e, w,
            w, e, w, e, e, e, w, e, w, w,
            w, e, e, e, e, e, w, e, e, w,
            w, e, e, e, e, e, w, e, e, w,
            w, e, e, e, e, e, w, e, e, w,
            w, e, e, w, e, e, w, e, e, w,
            w, e, e, e, e, e, w, e, e, w,
            w, w, w, w, w, w, w, w, w, w
        ];
        Self {
            width: 10,
            height: 10,
            data: temp_map,
        }
    }
}


impl Map {
    pub fn cell(&self, x: usize, y: usize) -> MapCell {
        assert!(x < self.width);
        assert!(y < self.height);
        self.data[y * self.width + x]
    }

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![MapCell::Empty; width * height]
        }
    }

    pub fn gen(width: usize, height: usize) -> Self {
        let mut map = Map::new(width, height);
        map.data = vec![MapCell::Wall([0.0, 0.0, 0.0]); width * height];

        for r in 1..map.height {
			for c in 1..map.width {
				if rand::random() {
					map.data[r * map.width + c - 1] = MapCell::Empty
				} else {
                    map.data[(r - 1) * map.width + c] = MapCell::Empty
				}
			}
		}
        //print!("{:?}", map);
        map
    }
    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn spawn_player_at(&self, coords: Coordinates, world: &mut World, player: u64) -> Coordinates {
        world.spawn((
            Player,
            player as UserID,
            Coordinates {
                x: coords.x,
                y: coords.y,
            }
        ));
        coords
    }
    pub fn spawn_player(&self, world: &mut World, player: u64) -> Coordinates {
        let mut available_coords: Vec<Coordinates> = Vec::new();
        for x in 0..self.width {
            for y in 0..self.height {
                if self.cell(x, y) == MapCell::Empty {
                    available_coords.push(Coordinates { x, y });
                }
            }
        }
        let rand_num: usize = rand::random::<usize>() % available_coords.len();
        self.spawn_player_at(available_coords[rand_num], world, player) 

    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MapCell {
    Empty,

    /// Wall with color
    Wall([f32; 3])
}


impl FromStr for MapCell {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "empty" => Ok(Self::Empty),
            "wall"  => Ok(Self::Wall([0.0, 0.0, 0.0])),
            _       => Err("Invalid MapElement in MapElement::from_str()".to_string())
        }
    }
}
