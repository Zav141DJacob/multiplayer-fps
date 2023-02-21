use std::str::FromStr;
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
        let e = MapCell::Empty;
        let w = MapCell::Wall(Wall::SolidColor([1.0, 1.0, 1.0]));
        let b = MapCell::Wall(Wall::Textured(Textured::Redstone));
        let t = MapCell::Wall(Wall::Textured(Textured::GrayBrick));
        let temp_map = vec![
            t, t, t, t, t, t, t, t, t, t,
            t, e, t, e, e, e, e, e, e, t,
            t, e, t, e, e, e, e, e, e, t,
            b, e, t, e, e, e, t, e, t, t,
            t, e, e, e, e, e, t, e, e, t,
            t, e, e, e, e, e, t, e, e, t,
            t, e, e, e, e, e, t, e, e, t,
            t, e, e, b, e, e, t, e, e, t,
            t, e, e, e, e, e, t, e, e, t,
            t, t, t, t, t, t, t, t, t, w
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
        map.data = vec![MapCell::Wall(Wall::SolidColor([1.0, 1.0, 1.0])); width * height];

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
    Wall(Wall),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Wall {
    SolidColor([f32; 3]),
    Textured(Textured),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Textured {
    Redstone,
    GrayBrick,
    RedBrick,
    Door,
    Green,
    Graystone,
}


impl FromStr for MapCell {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "empty" => Ok(Self::Empty),
            "wall"  => Ok(Self::Wall(Wall::SolidColor([0., 0., 0.]))),
            "brick"  => Ok(Self::Wall(Wall::Textured(Textured::RedBrick))),
            _       => Err("Invalid MapElement in MapElement::from_str()".to_string())
        }
    }
}
