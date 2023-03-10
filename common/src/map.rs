use glam::Vec2;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::ecs::components::Position;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub data: Vec<MapCell>,
}

#[rustfmt::skip::macros(vec)]
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
            data: vec![MapCell::Empty; width * height],
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

    pub fn random_empty_spot(&self) -> Position {
        let mut available_coords: Vec<Position> = Vec::new();
        for x in 0..self.width {
            for y in 0..self.height {
                if self.cell(x, y) == MapCell::Empty {
                    available_coords.push(Position(Vec2 {
                        x: x as f32 + 0.5,
                        y: y as f32 + 0.5,
                    }));
                }
            }
        }
        let rand_num: usize = rand::thread_rng().gen_range(0..available_coords.len());

        available_coords[rand_num]
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
            "wall"  => Ok(Self::Wall(Wall::SolidColor([0.0, 0.0, 0.0]))),
            "brick"  => Ok(Self::Wall(Wall::Textured(Textured::RedBrick))),
            _       => Err("Invalid MapElement in MapElement::from_str()".to_string())
        }
    }
}
