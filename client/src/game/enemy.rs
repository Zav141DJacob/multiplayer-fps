use notan::egui::Vec2;
use crate::game::Texture;
use crate::game::Position;

#[derive(Debug, Clone, Copy)]
pub struct Sprite {
    pub position: Position,
    pub texture_id: i32,
}

