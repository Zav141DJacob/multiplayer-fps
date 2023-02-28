use super::{pixels::Pixels, Player};
use common::map::{Map, Wall};
use glam::{Vec2, Vec4, Vec3};
use itertools::Position;
use notan::draw::{DrawImages, DrawTransform};
use notan::{
    draw::DrawShapes,
    prelude::{Color, Graphics, Texture},
};




pub struct GameUI {
    player_hp_max: usize,
    player_hp: usize,
    max_ammo: usize, 
    ammo: usize, 
    weapon_name: String,
    scale: Vec2,
    padding: Vec2,
    size: Vec2,
    border_size: Vec2
}




impl GameUI {
    pub fn new(player_hp: usize, player_hp_max: usize, max_ammo: usize, ammo: usize, weapon_name: String) -> Self {
    
        Self {
            player_hp_max,
            player_hp,
            max_ammo,
            ammo,
            weapon_name,
            scale: Vec2::new(2.0, 2.0),
            padding: Vec2::new(10.0, 10.0),
            size: Vec2::new(150.0, 10.0),
            border_size: Vec2::splat(4.0)
        }
    }

    pub fn set_scale(&mut self, scale: Vec2) {
        self.scale = scale;
    }

    pub fn set_size(&mut self, scale: Vec2) {
        self.size = scale;
    }

    pub fn set_padding(&mut self, padding: Vec2) {
        self.padding = padding;
    }

    pub fn set_health(&mut self, player_hp: usize) {
        self.player_hp = player_hp;
    }

    pub fn set_max_health(&mut self, player_hp_max: usize) {
        self.player_hp_max = player_hp_max;
    }

    pub fn draw_health(&self, draw: &mut notan::draw::Draw, width: usize, height: usize) {
        let mut healt_color = Color::GREEN;

        let proc = (self.player_hp as f32) / (self.player_hp_max as f32);
        if proc > 0.5 {
            healt_color = health_to_color_gradient((proc-0.5) / 0.5, Color::YELLOW, Color::GREEN);
        } else {
            healt_color = health_to_color_gradient((proc) / 0.5, Color::RED, Color::YELLOW);
        }

        let health_size = Vec2::new(self.size.x * proc, self.size.y);

        let position = Vec2::new(
            ((self.padding.x + self.border_size.x)  * self.scale.x),
            (height as f32) - ((self.padding.y + self.size.y+ self.border_size.y) * self.scale.y) );


        draw.rect( // Draw the health
            position.into(),
            (health_size * self.scale).into())

            .color(healt_color)
            .corner_radius(0.0);




        draw.rect( // Draw the lower thirds darker shading
            (position + self.size * self.scale * Vec2::new(0.0, 0.7) ).into(),
            (health_size * self.scale / Vec2::new(1.0, 3.0)).into())

            .color(Color::new(0.22, 0.22, 0.22, 0.2))
            .corner_radius(0.0);



        draw.rect(// Draw the upper thirds light shading
            (position).into(), 
            (health_size * self.scale / Vec2::new(1.0, 3.0)).into())

            .color(Color::new(1.0, 1.0, 1.0, 0.2))
            .corner_radius(0.0)
;


        draw.rect( // Draw border
            (position - self.border_size / 2.0).into(),
             (self.size * self.scale + self.border_size).into())

             .stroke_color(Color::BLACK)
             .fill_color(Color::TRANSPARENT)
             .stroke(self.border_size.x)
             .corner_radius(2.0);




    }
}



fn health_to_color_gradient(proc: f32, start_color: Color, into_color: Color) -> Color{
    let v0 = Vec3::new(start_color.r, start_color.g, start_color.b);
    let v1 = Vec3::new(into_color.r, into_color.g, into_color.b);

    let vec = v0 + proc * (v1 - v0);
    

    let mut res = Color::new(0.0, 0.0, 0.0, 1.0);

    res.r = vec.x;
    res.g = vec.y;
    res.b = vec.z;


    res
}