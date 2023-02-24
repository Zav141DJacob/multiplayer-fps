use common::defaults::PLAYER_SPEED;

use crate::game::*;

pub fn handle_keyboard_input(app: &App, w: f32, h: f32, p: (&mut Position, &mut Direction)) {
        if app.keyboard.is_down(KeyCode::W) {
            if (p.0.xy + p.1.xy * PLAYER_SPEED).x < 0.0 {
                p.0.xy.x = 0.0;
            } else if (p.0.xy + p.1.xy * PLAYER_SPEED).y < 0.0 {
                p.0.xy.y = 0.0;
            } else if (p.0.xy + p.1.xy * PLAYER_SPEED).x > w {
                p.0.xy.x = w;
            } else if (p.0.xy + p.1.xy * PLAYER_SPEED).y > h {
                p.0.xy.y = h;
            } else {
                p.0.xy += p.1.xy * PLAYER_SPEED;
            }
        }

        if app.keyboard.is_down(KeyCode::A) {
            if (p.0.xy - p.1.xy.perp() * PLAYER_SPEED).x < 0.0 {
                p.0.xy.x = 0.0;
            } else if (p.0.xy - p.1.xy.perp() * PLAYER_SPEED).y < 0.0 {
                p.0.xy.y = 0.0;
            } else if (p.0.xy - p.1.xy.perp() * PLAYER_SPEED).x > w {
                p.0.xy.x = w;
            } else if (p.0.xy - p.1.xy.perp() * PLAYER_SPEED).y > h {
                p.0.xy.y = h;
            } else {
                p.0.xy -= p.1.xy.perp() * PLAYER_SPEED;
            }
        }

        if app.keyboard.is_down(KeyCode::S) {
            if (p.0.xy - p.1.xy * PLAYER_SPEED).x < 0.0 {
                p.0.xy.x = 0.0;
            } else if (p.0.xy - p.1.xy * PLAYER_SPEED).y < 0.0 {
                p.0.xy.y = 0.0;
            } else if (p.0.xy - p.1.xy * PLAYER_SPEED).x > w {
                p.0.xy.x = w;
            } else if (p.0.xy - p.1.xy * PLAYER_SPEED).y > h {
                p.0.xy.y = h;
            } else {
                p.0.xy -= p.1.xy * PLAYER_SPEED;
            }
        }

        if app.keyboard.is_down(KeyCode::D) {
            if (p.0.xy + p.1.xy.perp() * PLAYER_SPEED).x < 0.0 {
                p.0.xy.x = 0.0;
            } else if (p.0.xy + p.1.xy.perp() * PLAYER_SPEED).y < 0.0 {
                p.0.xy.y = 0.0;
            } else if (p.0.xy + p.1.xy.perp() * PLAYER_SPEED).x > w {
                p.0.xy.x = w;
            } else if (p.0.xy + p.1.xy.perp() * PLAYER_SPEED).y > h {
                p.0.xy.y = h;
            } else {
                p.0.xy += p.1.xy.perp() * PLAYER_SPEED;
            }
        }

        if app.keyboard.is_down(KeyCode::Left) {
            p.1.xy = p.1.xy.rotate(Vec2::from_angle(-CAMERA_SENSITIVITY));
        }

        if app.keyboard.is_down(KeyCode::Right) {
            p.1.xy = p.1.xy.rotate(Vec2::from_angle(CAMERA_SENSITIVITY));
        }
}
