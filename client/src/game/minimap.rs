use common::map::{Map, Wall};
use notan::draw::DrawTransform;
use notan::{
    draw::DrawShapes,
    prelude::{Color, Graphics},
};
use glam::Vec2;
use crate::args::ARGS;
use crate::game::raycast::RayCaster;
use crate::game::texture::get_wall_texture;
use crate::game::texture::pixels::Pixels;

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
        let border_size = 3;
        let border_color = Color::GRAY;

        let map_ratio = 8;
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

    pub fn render_map(&mut self, gfx: &mut Graphics) {
        // Generate the texture for the map
        self.map_pixels.clear(self.floor_color);

        for x in 0..self.get_height() {
            for y in 0..self.get_width() {
                let map_x = (x as f32 / self.map_ratio as f32).floor() as i32;
                let map_y = (y as f32 / self.map_ratio as f32).floor() as i32;

                let color = match self.map.cell(map_x, map_y) {
                    common::map::MapCell::Empty => None,
                    common::map::MapCell::Wall(Wall::SolidColor(wall_color)) => {
                        Some(wall_color.into())
                    }
                    common::map::MapCell::Wall(Wall::Textured(wall_type)) => {
                        Some(get_wall_texture(wall_type).dominant().into())
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
        puffin::profile_function!();
        // Draw the border and map

        let minimap_translate = Vec2::new(
            (width as f32 - (self.get_width() as f32 * self.minimap_scale.x))
                - (self.minimap_pos.x) - self.border_size as f32,
            self.minimap_pos.y + self.border_size as f32,
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
        .color(self.border_color).corner_radius(2.0);

        self.map_pixels.draw(draw)
            .translate(minimap_translate.x, minimap_translate.y)
            .scale(self.minimap_scale.x, self.minimap_scale.y);
    }

    pub fn render_vision(
        &self,
        draw: &mut notan::draw::Draw,
        width: usize,
        _height: usize,
        vision_origin: Vec2,
        vision_color: Color,
        ray_caster: &RayCaster,
    ) {
        puffin::profile_function!();
        let rays = ray_caster.minimap_rays();
        let depths = ray_caster.depth_map();
        if ray_caster.minimap_rays().len() <= 1 {
            return
        }

        // Render vision form given rays
        let minimap_translate = Vec2::new(
            (width as f32 - (self.get_width() as f32 * self.minimap_scale.x))
                - (self.minimap_pos.x) - self.border_size as f32,
            self.minimap_pos.y + self.border_size as f32,
        );

        let mut path = draw.path();
        path.fill();
        path.fill_color(vision_color);

        let ray_start = minimap_translate + self.conver_ray_to_minimap_size(Vec2::new(vision_origin.x, vision_origin.y));
        path.move_to(ray_start.x, ray_start.y);

        let ray_end = minimap_translate + self.conver_ray_to_minimap_size(*rays.first().unwrap());
        path.line_to(ray_end.x, ray_end.y);

        let mut prev_slope = 0.0;
        let mut only_this = true; // Whether the previous i depth was already drawn
        const THRESH: f32 = 0.001;
        for i in 1..rays.len() {
            let prev_depth = depths[i-1];
            let this_depth = depths[i];

            let slope = this_depth - prev_depth;

            if (slope - prev_slope).abs() > THRESH {
                prev_slope = slope;

                if !only_this {
                    let ray_end = minimap_translate + self.conver_ray_to_minimap_size(rays[i-1]);
                    path.line_to(ray_end.x, ray_end.y);
                }
                let ray_end = minimap_translate + self.conver_ray_to_minimap_size(rays[i]);
                path.line_to(ray_end.x, ray_end.y);

                only_this = true;
            } else {
                only_this = false;
            }
        }

        if !only_this {
            let ray_end = minimap_translate + self.conver_ray_to_minimap_size(*rays.last().unwrap());
            path.line_to(ray_end.x, ray_end.y);
        }

        drop(path);

        if ARGS.debug {
            let ray_middle = self.conver_ray_to_minimap_size(rays[rays.len() / 2]) + minimap_translate;
            draw.line(ray_start.into(), ray_middle.into()).color(Color::GREEN);
        }
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
        puffin::profile_function!();
        // Render entities onto minimap
        let minimap_translate = Vec2::new(
            (width as f32 - (self.get_width() as f32 * self.minimap_scale.x))
                - (self.minimap_pos.x) - self.border_size as f32,
            self.minimap_pos.y + self.border_size as f32,
        );

        let entity_size = Vec2::new(2.0, 2.0) * self.minimap_scale;
        let entity_pos = minimap_translate + self.conver_ray_to_minimap_size(entity_pos);
        let entity_pos = entity_pos - (entity_size / 2.0);

        draw.rect(entity_pos.into(), entity_size.into())
            .color(entity_color);
    }

    pub fn conver_ray_to_minimap_size(&self, ray: Vec2) -> Vec2 {
        (ray * self.map_ratio as f32) * self.minimap_scale
    }
}
