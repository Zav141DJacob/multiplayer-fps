use notan::app::Color;

#[derive(Debug, Clone)]
pub struct Map {
    width: usize,
    height: usize,
    data: Vec<MapCell>,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            width: 10,
            height: 10,
            data: vec![MapCell::Empty; 100],
        }
    }
}

impl Map {
    pub fn cell(&self, x: usize, y: usize) -> MapCell {
        assert!(x < self.width);
        assert!(y < self.height);
        self.data[y * self.width + x]
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MapCell {
    Empty,

    /// Wall with color
    Wall(Color)
}