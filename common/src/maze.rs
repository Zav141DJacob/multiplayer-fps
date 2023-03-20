use bitflags::bitflags;
use rand::{Rng, thread_rng};
use rand::prelude::SliceRandom;
use crate::defaults::MAP_DEFAULT_WALL;
use crate::map::{Map, MapCell, Wall};

pub struct Maze {
    width: usize,
    height: usize,
    data: Vec<OpenWalls>,

    branching: f32,
}

bitflags! {
    struct OpenWalls: u8 {
        const UP    = 0b0001;
        const DOWN  = 0b0010;
        const LEFT  = 0b0100;
        const RIGHT = 0b1000;
    }
}

impl OpenWalls {
    fn opposite(self) -> Self {
        let a = (self & (Self::UP | Self::LEFT)).bits() << 1;
        let b = (self & (Self::DOWN | Self::RIGHT)).bits() >> 1;
        Self::from_bits_truncate(a | b)
    }
}

impl Maze {
    pub fn new(width: usize, height: usize, branching: f32) -> Self {
        assert!((0.0..=1.0).contains(&branching));
        let data = vec![OpenWalls::empty(); width * height];

        let mut maze = Self {
            width,
            height,
            data,
            branching,
        };

        maze.generate_maze();

        maze
    }

    fn cell(&self, x: usize, y: usize) -> OpenWalls {
        assert!(x < self.width);
        assert!(y < self.height);

        self.data[y * self.width + x]
    }

    fn generate_maze(&mut self) {
        let mut stack = vec![];

        let mut rng = thread_rng();
        {
            // Choose random starting point
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);
            stack.push((x, y));
        }

        while !stack.is_empty() {
            let (cx, cy) = if rng.gen_bool(self.branching as f64) {
                stack.pop().unwrap()
            } else {
                let i = rng.gen_range(0..stack.len());
                stack.remove(i)
            };

            let mut neighbours = 0;

            // Up
            if let Some(ny) = cy.checked_sub(1) {
                if self.tunnel(cx, cy, cx, ny, OpenWalls::UP) {
                    neighbours += 1;
                    stack.push((cx, ny));
                }
            }

            // Down
            if let Some(ny) = Some(cy + 1).filter(|&ny| ny < self.width) {
                if self.tunnel(cx, cy, cx, ny, OpenWalls::DOWN) {
                    neighbours += 1;
                    stack.push((cx, ny));
                }
            }

            // Left
            if let Some(nx) = cx.checked_sub(1) {
                if self.tunnel(cx, cy, nx, cy, OpenWalls::LEFT) {
                    neighbours += 1;
                    stack.push((nx, cy));
                }
            }

            // Right
            if let Some(nx) = Some(cx + 1).filter(|&nx| nx < self.width) {
                if self.tunnel(cx, cy, nx, cy, OpenWalls::RIGHT) {
                    neighbours += 1;
                    stack.push((nx, cy));
                }
            }

            let len = stack.len();
            stack[len-neighbours..len].shuffle(&mut rng);
        }
    }

    fn tunnel(&mut self, cx: usize, cy: usize, nx: usize, ny: usize, direction: OpenWalls) -> bool {
        if !self.data[ny * self.width + nx].is_empty() {
            // Return false if next is already visited
            return false;
        }

        self.data[cy * self.width + cx] |= direction;
        self.data[ny * self.width + nx] |= direction.opposite();

        true
    }
}

impl From<Maze> for Map {
    fn from(maze: Maze) -> Self {
        let mut map = Map::new(maze.width * 2 + 1, maze.height * 2 + 1);
        map.data.fill(MapCell::Wall(Wall::Textured(MAP_DEFAULT_WALL)));

        for maze_y in 0..maze.height {
            for maze_x in 0..maze.width {
                let map_x = maze_x * 2 + 1;
                let map_y = maze_y * 2 + 1;

                let cell = map.cell_mut(map_x, map_y);
                *cell = MapCell::Empty;

                let walls = maze.cell(maze_x, maze_y);

                if walls.contains(OpenWalls::UP) {
                    let cell = map.cell_mut(map_x, map_y - 1);
                    *cell = MapCell::Empty;
                }

                if walls.contains(OpenWalls::DOWN) {
                    let cell = map.cell_mut(map_x, map_y + 1);
                    *cell = MapCell::Empty;
                }

                if walls.contains(OpenWalls::LEFT) {
                    let cell = map.cell_mut(map_x - 1, map_y);
                    *cell = MapCell::Empty;
                }

                if walls.contains(OpenWalls::RIGHT) {
                    let cell = map.cell_mut(map_x + 1, map_y);
                    *cell = MapCell::Empty;
                }
            }
        }

        map
    }
}
