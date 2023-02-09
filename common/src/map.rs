
use rand::{thread_rng, Rng};
use std::{str::FromStr};
use serde::{Deserialize, Serialize};

use crate::defaults::{MAP_WIDTH, MAP_HEIGHT};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
    width: usize,
    height: usize,
    data: Vec<MapCell>,
}

impl Default for Map {
    fn default() -> Self {
        Map::new(MAP_WIDTH, MAP_HEIGHT)
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
        return map
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
