use glam::Vec2;
use rand::{thread_rng, Rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::ecs::components::Position;

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
    pub fn cell(&self, x: i32, y: i32) -> MapCell {
        if x > self.width as i32 || y > self.height as i32 {
            return MapCell::Empty;
        }
        if x < 0 || y < 0 {
            return MapCell::Empty;
        }
        // assert!(y < self.height);
        self.data[(y as usize) * self.width + x as usize]
    }

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![MapCell::Empty; width * height],
        }
    }

    fn set(&mut self, x: usize, y: usize, value: MapCell) {
        assert!(x < self.width);
        assert!(y < self.height);
        self.data[y * self.width + x] = value;
    }

    fn divide(
        &mut self,
        x1: usize,
        x2: usize,
        y1: usize,
        y2: usize,
        mut skiplist: Vec<Skip>,
        horiz: bool,
    ) {
        if x2 <= x1 || y2 <= y1 {
            return;
        }

        let width = x2 - x1;
        let height = y2 - y1;

        if horiz && width < 2 || !horiz && height < 2 {
            return;
        }

        let mut rng = thread_rng();

        if horiz {
            let wall_y = y1 + (height / 2);
            let door_x1 = x1 + rng.gen_range(0..width);
            let door_x2 = x1 + rng.gen_range(0..width);
            skiplist.push(Skip {
                x: door_x1,
                y: wall_y,
            });
            skiplist.push(Skip {
                x: door_x2,
                y: wall_y,
            });
            if wall_y > 0 {
                skiplist.push(Skip {
                    x: door_x1,
                    y: wall_y - 1,
                });
                skiplist.push(Skip {
                    x: door_x2,
                    y: wall_y - 1,
                });
            }
            if wall_y < height - 1 {
                skiplist.push(Skip {
                    x: door_x1,
                    y: wall_y + 1,
                });
                skiplist.push(Skip {
                    x: door_x2,
                    y: wall_y + 1,
                });
            }
            for x in x1..x2 {
                if !skiplist.contains(&Skip { x, y: wall_y }) {
                    self.set(
                        x,
                        wall_y,
                        MapCell::Wall(Wall::Textured(Textured::GrayBrick)),
                    );
                }
            }
            self.divide(x1, x2, y1, wall_y, skiplist.clone(), !horiz);
            self.divide(x1, x2, wall_y + 1, y2, skiplist.clone(), !horiz);
        } else {
            let wall_x = x1 + (width / 2);
            let door_y1 = y1 + rng.gen_range(0..height);
            let door_y2 = y1 + rng.gen_range(0..height);
            skiplist.push(Skip {
                x: wall_x,
                y: door_y1,
            });
            skiplist.push(Skip {
                x: wall_x,
                y: door_y2,
            });
            if wall_x > 0 {
                skiplist.push(Skip {
                    x: wall_x - 1,
                    y: door_y1,
                });
                skiplist.push(Skip {
                    x: wall_x - 1,
                    y: door_y2,
                });
            }
            if wall_x < width - 1 {
                skiplist.push(Skip {
                    x: wall_x + 1,
                    y: door_y1,
                });
                skiplist.push(Skip {
                    x: wall_x + 1,
                    y: door_y2,
                });
            }
            for y in y1..y2 {
                if !skiplist.contains(&Skip { x: wall_x, y }) {
                    self.set(
                        wall_x,
                        y,
                        MapCell::Wall(Wall::Textured(Textured::GrayBrick)),
                    );
                }
            }
            self.divide(x1, wall_x, y1, y2, skiplist.clone(), !horiz);
            self.divide(wall_x + 1, x2, y1, y2, skiplist.clone(), !horiz);
        }
    }

    pub fn gen(width: usize, height: usize) -> Map {
        let mut map = Map::new(width, height);
        let skiplist: Vec<Skip> = Vec::new();
        map.divide(0, width, 0, height, skiplist, true);
        for r in 0..map.width {
            for c in 0..map.height {
                if r == 0 || c == 0 || r == map.height - 1 || c == map.width - 1 {
                    map.set(r, c, MapCell::Wall(Wall::Textured(Textured::GrayBrick)))
                }
            }
        }

        //Self::console_log_map(&map);
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
            "wall" => Ok(Self::Wall(Wall::SolidColor([0.0, 0.0, 0.0]))),
            "brick" => Ok(Self::Wall(Wall::Textured(Textured::RedBrick))),
            _ => Err("Invalid MapElement in MapElement::from_str()".to_string()),
        }
    }
}
