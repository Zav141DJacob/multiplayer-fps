use crate::game::Position;
use glam::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Sprite {
    pub position: Position,
    pub texture_id: i32,
}

pub fn add_enemies() -> Vec<Sprite> {
    let enemies_init = vec![(3.834, 2.765, 0), (5.323, 5.365, 2), (8.123, 8.265, 3)];
    let mut enemies = vec![];
    for i in enemies_init {
        let enemy = Sprite {
            texture_id: i.2,
            position: Position {
                xy: Vec2::new(i.0, i.1),
            },
        };
        enemies.push(enemy);
    }
    enemies
}
