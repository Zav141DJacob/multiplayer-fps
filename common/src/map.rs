

use std::{str::FromStr};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
    width: usize,
    height: usize,
    data: Vec<MapCell>,
}

impl Default for Map {
    fn default() -> Self {
        let w = MapCell::Wall([0.0, 0.0, 0.0]);
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
    pub fn gen() -> Self {
        // TODO:
        //  Georgis algorithm code goes here
        Self::default()
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
