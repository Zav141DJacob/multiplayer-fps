use common::defaults::PLAYER_MAX_HP;
use common::gun::Gun;
use glam::{Vec2, Vec3};
use notan::draw::{CreateFont, DrawShapes, DrawTextSection, Font};
use notan::prelude::{Color, Graphics};

pub struct GameUI {
    game_state: GameUiState,
    scale: Vec2,
    padding: Vec2,
    size: Vec2,
    border_size: Vec2,
    font: Font,
}

pub struct GameUiState {
    pub player_hp_max: f32,
    pub player_hp: f32,
    pub weapon_name: String,
    pub max_ammo: usize,
    pub ammo: usize,
}

impl GameUiState {
    pub fn new() -> Self {
        GameUiState {
            player_hp_max: PLAYER_MAX_HP,
            player_hp: PLAYER_MAX_HP,
            weapon_name: Gun::Pistol.to_string(),
            max_ammo: Gun::Pistol.max_ammo(),
            ammo: Gun::Pistol.max_ammo(),
        }
    }
}

impl GameUI {
    pub fn new(game_state: GameUiState, gfx: &mut Graphics) -> Self {
        let font = gfx
            .create_font(include_bytes!("../../assets/Ubuntu-B.ttf"))
            .unwrap();

        Self {
            game_state,
            scale: Vec2::new(2.0, 2.0),
            padding: Vec2::new(10.0, 10.0),
            size: Vec2::new(150.0, 10.0),
            border_size: Vec2::splat(4.0),
            font,
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

    pub fn set_game_state(&mut self, game_state: GameUiState) {
        self.game_state = game_state
    }

    pub fn draw_health(&self, draw: &mut notan::draw::Draw, _width: usize, height: usize) {
        let proc = self.game_state.player_hp / self.game_state.player_hp_max;
        let health_color = if proc > 0.5 {
            health_to_color_gradient((proc - 0.5) / 0.5, Color::YELLOW, Color::GREEN)
        } else {
            health_to_color_gradient((proc) / 0.5, Color::RED, Color::YELLOW)
        };

        let health_size = Vec2::new(self.size.x * proc, self.size.y);

        let position = Vec2::new(
            (self.padding.x + self.border_size.x) * self.scale.x,
            (height as f32) - ((self.padding.y + self.size.y + self.border_size.y) * self.scale.y),
        );

        draw.rect(
            // Draw the health
            position.into(),
            (health_size * self.scale).into(),
        )
        .color(health_color)
        .corner_radius(0.0);

        draw.rect(
            // Draw the lower thirds darker shading
            (position + self.size * self.scale * Vec2::new(0.0, 0.7)).into(),
            (health_size * self.scale / Vec2::new(1.0, 3.0)).into(),
        )
        .color(Color::new(0.22, 0.22, 0.22, 0.2))
        .corner_radius(0.0);

        draw.rect(
            // Draw the upper thirds light shading
            (position).into(),
            (health_size * self.scale / Vec2::new(1.0, 3.0)).into(),
        )
        .color(Color::new(1.0, 1.0, 1.0, 0.2))
        .corner_radius(0.0);

        draw.rect(
            // Draw border
            (position - self.border_size / 2.0).into(),
            (self.size * self.scale + self.border_size).into(),
        )
        .stroke_color(Color::BLACK)
        .fill_color(Color::TRANSPARENT)
        .stroke(self.border_size.x)
        .corner_radius(2.0);
    }

    pub fn draw_weapon_stats(&self, draw: &mut notan::draw::Draw, width: usize, height: usize) {
        let position = Vec2::new(width as f32 - 200.0, height as f32 - 50.0);


        let mut ammo_text = "∞".to_string();

        if self.game_state.max_ammo != 0 {
            ammo_text = format!(
                "{:0>3} / {:0>3}",
                self.game_state.ammo, self.game_state.max_ammo
            );
        }

        draw.text(&self.font, &self.game_state.weapon_name)
            .position(position.x, position.y);
        draw.text(&self.font, &ammo_text)
            .position(position.x + 120.0, position.y);

        let padding = 10.0 / self.game_state.max_ammo as f32;
        let bullet_bar_size = Vec2::new(170.0 / self.game_state.max_ammo as f32, 5.0);

        for i in 0..self.game_state.max_ammo {
            let mut color = Color::WHITE;

            if (self.game_state.ammo) as f32 <= ((self.game_state.max_ammo as f32) * 0.25) {
                color = Color::RED
            } else if (self.game_state.ammo) as f32 <= ((self.game_state.max_ammo as f32) * 0.6) {
                color = Color::ORANGE
            }

            if self.game_state.max_ammo - i > self.game_state.ammo {
                color = Color::GRAY;
            }

            draw.rect(
                (
                    position.x + (bullet_bar_size.x + padding) * i as f32,
                    position.y + 20.0,
                ),
                bullet_bar_size.into(),
            )
            .color(color)
            .corner_radius(2.0);
        }
    }
}

fn health_to_color_gradient(proc: f32, start_color: Color, into_color: Color) -> Color {
    let v0 = Vec3::new(start_color.r, start_color.g, start_color.b);
    let v1 = Vec3::new(into_color.r, into_color.g, into_color.b);

    let vec = v0 + proc * (v1 - v0);

    let mut res = Color::new(0.0, 0.0, 0.0, 1.0);

    res.r = vec.x;
    res.g = vec.y;
    res.b = vec.z;

    res
}
