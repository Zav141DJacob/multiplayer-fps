use super::{pixels::Pixels, Player};
use common::map::{Map, Wall};
use notan::draw::{DrawImages, DrawTransform};
use notan::{
    draw::DrawShapes,
    prelude::{Color, Graphics, Texture},
};
use glam::Vec2;

pub struct Minimap {
    border_size: usize,
    border_color: Color,
    map_ratio: usize,
    floor_color: Color,

    minimap_scale: Vec2,
    minimap_pos: Vec2,

    map_pixels: Pixels,

    map: Map,
}

impl Minimap {
    pub fn new(map: Map, gfx: &mut Graphics) -> Self {
        let border_size = 2;
        let border_color = Color::GRAY;

        let map_ratio = 10;
        let floor_color = Color::BLACK;

        let minimap_scale = Vec2::new(2.0, 2.0);
        let minimap_pos = Vec2::new(10.0, 10.0);

        let map_pixels = Pixels::new(
            map.get_width() * map_ratio,
            map.get_width() * map_ratio,
            gfx,
        );


        Self {
            border_size,
            border_color,
            map_ratio,
            floor_color,

            minimap_scale,
            minimap_pos,

            map_pixels,

            map,
        }
    }

    pub fn get_map_texture(&self) -> &Texture {
        self.map_pixels.texture()
    }

    pub fn render_map(&mut self, gfx: &mut Graphics) {
        // Generate the texture for the map
        self.map_pixels.clear(self.floor_color);

        for x in 0..self.get_height() {
            for y in 0..self.get_width() {
                let map_x = (x as f32 / self.map_ratio as f32).floor() as usize;
                let map_y = (y as f32 / self.map_ratio as f32).floor() as usize;

                let color = match self.map.cell(map_x, map_y) {
                    common::map::MapCell::Empty => None,
                    common::map::MapCell::Wall(Wall::SolidColor(wall_color)) => {
                        Some(wall_color.into())
                    }
                    common::map::MapCell::Wall(Wall::Textured(_)) => {
                        Some(Color::new(1.0, 0.0, 1.0, 1.0))
                    }
                };

                if let Some(mut color) = color {
                    if x % self.map_ratio == 0
                        || y % self.map_ratio == 0
                        || y % self.map_ratio == self.map_ratio - 1
                        || x % self.map_ratio == self.map_ratio - 1
                    {
                        color = Color::BLACK
                    }

                    self.map_pixels.set_color(x, y, color);
                }
            }
        }

        self.map_pixels.flush(gfx);
    }

    pub fn set_floor_color(&mut self, color: Color) {
        self.floor_color = color
    }

    pub fn set_border_color(&mut self, color: Color) {
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
        self.map.get_width() * self.map_ratio
    }
    pub fn get_height(&self) -> usize {
        self.map.get_height() * self.map_ratio
    }

    pub fn draw(&self, draw: &mut notan::draw::Draw, width: usize, _height: usize) {
        // Draw the border and map

        let minimap_translate = Vec2::new(
            (width as f32 - (self.get_width() as f32 * self.minimap_scale.x))
                - (self.minimap_pos.x),
            self.minimap_pos.y,
        );

        draw.rect(
            (
                minimap_translate.x - self.border_size as f32,
                minimap_translate.y - self.border_size as f32,
            ),
            (
                self.minimap_scale.x * self.get_width() as f32 + (self.border_size * 2) as f32,
                self.minimap_scale.y * self.get_height() as f32 + (self.border_size * 2) as f32,
            ),
        )
        .color(self.border_color);

        draw.image(self.get_map_texture())
            .translate(minimap_translate.x, minimap_translate.y)
            .scale(self.minimap_scale.x, self.minimap_scale.y);
    }

    pub fn render_vision(
        &self,
        draw: &mut notan::draw::Draw,
        width: usize,
        _height: usize,
        vision_origin: Vec2,
        mut vision_color: Color,
        rays: Vec<Vec2>,
    ) {
        // Render vision form given rays
        let minimap_translate = Vec2::new(
            (width as f32 - (self.get_width() as f32 * self.minimap_scale.x))
                - (self.minimap_pos.x),
            self.minimap_pos.y,
        );

        let ray_start = minimap_translate + self.conver_ray_to_minimap_size(Vec2::new(vision_origin.x, vision_origin.y));
        let ray_middle = self.conver_ray_to_minimap_size(rays[rays.len()/2]) + minimap_translate;
        for (i, mut ray_end) in rays.clone().into_iter().enumerate() {
            ray_end = self.conver_ray_to_minimap_size(ray_end);

            ray_end = minimap_translate + ray_end;

            if i < (rays.len() / 2) {
                vision_color.a = i as f32 / (rays.len() as f32);
            } else {
                vision_color.a =  1.0 - (i as f32 / (rays.len() as f32 ));
            }

            vision_color.a = vision_color.a.powf(1.0 / 1.1);

            draw.line(ray_start.into(), ray_end.into()).color(vision_color);




        }
        draw.line(ray_start.into(), ray_middle.into()).color(Color::GREEN);

    }

    pub fn render_player_location(
        &self,
        draw: &mut notan::draw::Draw,
        width: usize,
        height: usize,
        player_pos: Vec2,
        player_color: Color,
    ) {
        self.render_entity_location(
            draw,
            width,
            height,
            Vec2::new(player_pos.x, player_pos.y),
            player_color,
        );
    }

    pub fn render_entity_location(
        &self,
        draw: &mut notan::draw::Draw,
        width: usize,
        _height: usize,
        entity_pos: Vec2,
        entity_color: Color,
    ) {
        // Render entities onto minimap
        let minimap_translate = Vec2::new(
            (width as f32 - (self.get_width() as f32 * self.minimap_scale.x))
                - (self.minimap_pos.x),
            self.minimap_pos.y,
        );

        let entity_size = Vec2::new(1.0, 1.0) * self.minimap_scale;
        let entity_pos = minimap_translate + self.conver_ray_to_minimap_size(entity_pos);
        let entity_pos = entity_pos - (entity_size / 2.0);

        draw.rect(entity_pos.into(), entity_size.into())
            .color(entity_color);
    }

    pub fn conver_ray_to_minimap_size(&self, ray: Vec2) -> Vec2 {
        (ray * self.map_ratio as f32) * self.minimap_scale
    }
}
