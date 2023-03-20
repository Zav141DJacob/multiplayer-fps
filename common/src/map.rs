use glam::Vec2;
use rand::{thread_rng, seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

use rand::distributions::Standard;
use rand::prelude::Distribution;
use crate::defaults::{MAP_BRANCHING, MAP_DEFAULT_WALL, MAP_OPENNESS, MAP_SECTOR_COUNT, MAP_SECTOR_MAX_SIZE, MAP_SECTOR_MIN_SIZE};

use crate::ecs::components::Position;
use crate::maze::Maze;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub data: Vec<MapCell>,
}

#[derive(Debug, Clone, PartialEq)]
struct Skip {
    pub x: usize,
    pub y: usize,
}

#[rustfmt::skip::macros(vec)]
impl Default for Map {
    fn default() -> Self {
        let e = MapCell::Empty;
        let w = MapCell::Wall(Wall::SolidColor([1.0, 1.0, 1.0]));
        let b = MapCell::Wall(Wall::Textured(MAP_DEFAULT_WALL));
        let t = b;
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
    pub fn cell(&self, x: i32, y: i32) -> MapCell {
        if x >= self.width as i32 || y >= self.height as i32 {
            return MapCell::Empty;
        }
        if x < 0 || y < 0 {
            return MapCell::Empty;
        }
        // assert!(y < self.height);
        self.data[(y as usize) * self.width + x as usize]
    }

    pub fn cell_mut(&mut self, x: usize, y: usize) -> &mut MapCell {
        assert!(x < self.width);
        assert!(y < self.height);
        &mut self.data[y * self.width + x]
    }

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![MapCell::Empty; width * height],
        }
    }

    pub fn gen(width: usize, height: usize) -> Map {
        assert!(width >= 3 && width % 2 == 1);
        assert!(height >= 3 && height % 2 == 1);
        let maze = Maze::new(width / 2, height / 2, MAP_BRANCHING);
        let mut map = Map::from(maze);

        // Remove some of the walls to make the map more open
        let all_walls: Vec<_> = (1..map.height-1).flat_map(|y| {
            let x = y % 2;
            (1+x..map.width-1).step_by(2).map(move |x| (x, y))
        })
            .filter(|&(x, y)| !matches!(map.cell(x as i32, y as i32), MapCell::Empty))
            .collect();

        let walls_to_remove = (all_walls.len() as f32 * MAP_OPENNESS) as usize;

        let rng = &mut thread_rng();
        for &(x, y) in all_walls.choose_multiple(rng, walls_to_remove) {
            *map.cell_mut(x, y) = MapCell::Empty;
        }

        // Make some walls in the map a different texture
        for _ in 0..MAP_SECTOR_COUNT {
            let tex = loop {
                let tex = rng.gen();
                if tex != MAP_DEFAULT_WALL {
                    break tex;
                }
            };

            let width = rng.gen_range(MAP_SECTOR_MIN_SIZE..MAP_SECTOR_MAX_SIZE);
            let height = rng.gen_range(MAP_SECTOR_MIN_SIZE..MAP_SECTOR_MAX_SIZE);

            let x = rng.gen_range(0..=map.width-width);
            let y = rng.gen_range(0..=map.height-height);

            for y in y..y+height {
                for x in x..x+width {
                    if let MapCell::Wall(Wall::Textured(this_tex)) = map.cell_mut(x, y) {
                        *this_tex = tex;
                    }
                }
            }
        }

        map
    }

    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn console_log_map(map: &Map) {
        let mut i = 0;
        for x in &map.data {
            i += 1;
            if x.ne(&MapCell::Empty) {
                print!("X");
            } else {
                print!(" ");
            }
            if i % map.width == 0 {
                println!();
            }
        }
    }

    pub fn random_empty_spot(&self) -> Option<Position> {
        let mut available_coords: Vec<Position> = Vec::new();
        for x in 0..self.width {
            for y in 0..self.height {
                if self.cell(x as i32, y as i32) == MapCell::Empty {
                    available_coords.push(Position(Vec2 {
                        x: x as f32 + 0.5,
                        y: y as f32 + 0.5,
                    }));
                }
            }
        }

        available_coords.choose(&mut thread_rng()).copied()
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
impl Default for Wall {
    fn default() -> Self {
        Wall::SolidColor([0.0; 3])
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Textured {
    Brick1,
    Brick2,
    Door,
    Industrial,
    Rocky,
    Techy,
    Urban,
    Wood,
}

impl Distribution<Textured> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Textured {
        let i = rng.gen_range(0..=6);
        match i {
            0 => Textured::Brick1,
            1 => Textured::Brick2,
            2 => Textured::Industrial,
            3 => Textured::Rocky,
            4 => Textured::Techy,
            5 => Textured::Urban,
            6 => Textured::Wood,
            _ => unreachable!(),
        }
    }
}
