use std::f32::consts::PI;

use notan::egui::epaint::ahash::{HashMap, HashMapExt};

use super::sampler::TextureSampler;

pub struct AnimatedTexture {
    sprite_sheet: &'static [TextureSampler],
    states: HashMap<String, AnimationMap>,
}

struct AnimationMap {
    states: Vec<Vec<usize>>,
    frame_time: f32,
}

pub struct AnimatedTextureState {
    tex: &'static AnimatedTexture,
    cur_state: &'static AnimationMap,
    cur_state_str: String,

    cur_frame: usize,
    frame_time_mult: f32,
    current_frame_accumulator: f32,
}

impl AnimatedTexture {
    pub fn new(sprite_sheet: &'static [TextureSampler]) -> Self {
        Self {
            sprite_sheet,
            states: HashMap::new(),
        }
    }

    pub fn register_state(mut self, name: &str, frame_time: f32, states: Vec<Vec<usize>>) -> Self {
        self.states
            .insert(name.to_string(), AnimationMap { states, frame_time });
        self
    }

    pub fn get_state(&'static self, initial_state: &str) -> AnimatedTextureState {
        AnimatedTextureState {
            tex: self,
            cur_state: self
                .states
                .get(initial_state)
                .expect("No animation states exist, with that name, or empty"),
            cur_state_str: initial_state.to_string(),
            cur_frame: 0,
            frame_time_mult: 1.0,
            current_frame_accumulator: 0.0,
        }
    }
}

impl AnimatedTextureState {
    pub fn set_state(&mut self, name: &str, speed_mult: f32) {
        if self.cur_state_str == name {
            return;
        }

        self.cur_state_str = name.to_string();
        self.cur_state = self
            .tex
            .states
            .get(name)
            .expect("No animation states exist, with that name, or empty");
        self.cur_frame = 0;
        self.current_frame_accumulator = 0.0;
        self.frame_time_mult = 1.0 / speed_mult;
    }

    pub fn get_sprite(&mut self, look_angle: f32, cur_time: f32) -> &'static TextureSampler {
        self.get_cur_animation_frame(cur_time);

        &self.tex.sprite_sheet[self.get_angled_frame(look_angle)]
    }

    fn get_cur_animation_frame(&mut self, cur_time: f32) {
        self.current_frame_accumulator += cur_time;
        let frame_time = self.cur_state.frame_time * self.frame_time_mult;

        if self.current_frame_accumulator >= frame_time {
            self.current_frame_accumulator -= frame_time;
            self.cur_frame += 1;
            self.cur_frame %= self.cur_state.states.len();
        }
    }

    fn get_angled_frame(&mut self, look_angle: f32) -> usize {
        let cur_frame = &self.cur_state.states[self.cur_frame];
        let cur_frame_vec_size = cur_frame.len() as f32;

        let index = ((look_angle / (2.0 * PI)) * cur_frame_vec_size).round() as i32;

        let index = index.rem_euclid(cur_frame_vec_size as i32) as usize;

        cur_frame[index]
    }
}
