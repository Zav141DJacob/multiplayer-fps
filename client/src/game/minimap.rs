use common::map::Map;
use notan::{prelude::{Color, Graphics, Texture}, graphics::color, egui::Vec2, draw::DrawShapes};
use notan::draw::{CreateDraw, DrawImages, DrawTransform};
use super::pixels::Pixels;


pub struct Minimap {
    border_size: usize,
    border_color: Color,
    map_ratio: usize,
    floor_color: Color,

    minimap_scale: Vec2,
    minimap_pos: Vec2,

    map_pixels: Pixels,
    player_pixels: Pixels,

    map: Map,

}

impl Minimap {
    pub fn new(map: Map, gfx: &mut Graphics) -> Self{

        let border_size = 2;
        let border_color = Color::GRAY;

        let map_ratio = 10;
        let floor_color = Color::BLACK;

        let minimap_scale = Vec2::new(2.0, 2.0);
        let minimap_pos = Vec2::new(10.0, 10.0);

        let mut map_pixels = Pixels::new(map.get_width() * map_ratio, map.get_width() * map_ratio, gfx);



        let player_pixels = Pixels::new(map.get_width() * 10, map.get_width() * 10, gfx);

        Self {
            border_size,
            border_color,
            map_ratio,
            floor_color,

            minimap_scale,
            minimap_pos,

            map_pixels,
            player_pixels,

            map,

        }
    }

    pub fn get_map_texture(&self) -> &Texture {
        self.map_pixels.texture()
    }

    pub fn render_map(&mut self, gfx: &mut Graphics) { // Generate the texture for the map
        self.map_pixels.clear(self.floor_color);

        for x in 0..self.get_height() {
            for y in 0..self.get_width() {
                let map_x = (x as f32 / self.map_ratio as f32).floor() as usize;
                let map_y = (y as f32 / self.map_ratio as f32).floor() as usize;

                match self.map.cell(map_x, map_y) {
                    common::map::MapCell::Empty => {},
                    common::map::MapCell::Wall(wall_color) => {
                        let mut color:Color = wall_color.into();

                        if (
                            x % self.map_ratio == 0 || 
                            y % self.map_ratio == 0 ||
                            y % self.map_ratio == self.map_ratio -1 ||
                            x % self.map_ratio == self.map_ratio -1
                            
                        ) {
                            color = Color::BLACK
                        }

                        self.map_pixels.set_color(x, y, color);
                    },
                }

            }
        }

        self.map_pixels.flush(gfx);
    }

    pub fn set_floor_color(&mut self, color:Color) {
        self.floor_color = color
    }

    pub fn set_border_color(&mut self, color:Color) {
        self.border_color = color
    }
    
    pub fn set_border_size(&mut self, size: usize) {
        self.border_size = size
    }

    pub fn set_minimap_pos(&mut self, pos: Vec2) {
        self.minimap_pos = pos
    }

    pub fn set_minimap_scale(&mut self, scale: Vec2) {
        self.minimap_scale = scale
    }

    pub fn set_map(&mut self, map: Map) {
        self.map = map
    }

    pub fn get_width(&self) -> usize {
        self.map.get_width()*self.map_ratio
    }
    pub fn get_height(&self) -> usize {
        self.map.get_height()*self.map_ratio
    }


    pub fn draw(&self, draw: &mut notan::draw::Draw, width: usize, height: usize) { // Draw the border and map

        let minimap_translate = Vec2::new(
            (width as f32 - (self.get_width() as f32 * self.minimap_scale.x)) - (self.minimap_pos.x ),
            self.minimap_pos.y
        );
        
        draw.rect((minimap_translate.x - self.border_size as f32, minimap_translate.y - self.border_size as f32), (
            self.minimap_scale.x * self.get_width() as f32 + (self.border_size * 2) as f32,
            self.minimap_scale.y * self.get_height() as f32 + (self.border_size * 2) as f32
        )).color(self.border_color);

        draw.image(self.get_map_texture())
            .translate(minimap_translate.x, minimap_translate.y)
            .scale(self.minimap_scale.x, self.minimap_scale.y);

    }
}