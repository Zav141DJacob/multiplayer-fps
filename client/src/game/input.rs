use std::f32::consts::PI;
use glam::{IVec2, Vec2};
use notan::Event;
use notan::prelude::{App, KeyCode, MouseButton};
use common::ecs::components::InputState;

const MOUSE_SENSITIVITY: f32 = 3.0 / 10000.0;
const KB_LOOK_SENSITIVITY: f32 = 2.5;
const UP_DOWN_ANGLE_CLAMP: f32 = 45.0 / 180.0 * PI;

pub struct InputHandler {
    dirty: bool,
    state: InputState,

    up_down_angle: f32,

    mouse_locked: bool,
    slow_look: bool,

    #[cfg(feature = "mouse-look")]
    mouse: MouseController,
}

impl InputHandler {
    pub fn new(_app: &mut App) -> Self {
        Self {
            dirty: false,
            state: Default::default(),
            up_down_angle: 0.0,
            mouse_locked: false,
            slow_look: false,
            #[cfg(feature = "mouse-look")]
            mouse: MouseController::new(_app),
        }
    }

    /// Returns the current state if it's dirty
    pub fn take_state(&mut self) -> Option<InputState> {
        let state = self.dirty.then_some(self.state);
        self.dirty = false;
        state
    }

    /// Peeks current state regardless of dirty flag
    pub fn peek_state(&self) -> InputState {
        self.state
    }

    pub fn tick(&mut self, app: &mut App) {
        let dt = app.timer.delta_f32();
        self.up_down_angle = lerp(self.up_down_angle, 0.0, 5.0 * dt);

        let mut look = Vec2::ZERO;
        if app.keyboard.is_down(KeyCode::Right) {
            look.x -= 1.0;
        }
        if app.keyboard.is_down(KeyCode::Left) {
            look.x += 1.0;
        }
        if app.keyboard.is_down(KeyCode::Up) {
            look.y += 1.0;
        }
        if app.keyboard.is_down(KeyCode::Down) {
            look.y -= 1.0;
        }

        if look == Vec2::ZERO {
            return
        }

        if self.slow_look {
            look /= 3.0;
        }

        self.apply_look_delta(look * dt * KB_LOOK_SENSITIVITY);
        self.dirty = true;

    }

    pub fn up_down_angle(&self) -> f32 {
        self.up_down_angle
    }

    #[cfg(feature = "mouse-look")]
    pub fn mouse_locked(&self) -> bool {
        self.mouse_locked
    }

    #[cfg(not(feature = "mouse-look"))]
    pub fn mouse_locked(&self) -> bool {
        false
    }

    /// Handles a given event.
    pub fn handle_event(&mut self, event: Event) {
        let state_dirtied = match event {
            Event::MouseWheel { .. } => return,
            Event::KeyDown {key} => {
                self.handle_key(key, true)
            }
            Event::KeyUp {key} => {
                self.handle_key(key, false)
            }
            #[cfg(feature = "mouse-look")]
            Event::MouseDown { button, .. } => {
                self.handle_click(button, true)
            }
            #[cfg(feature = "mouse-look")]
            Event::MouseUp { button, .. } => {
                self.handle_click(button, false)
            },
            #[cfg(feature = "mouse-look")]
            Event::MouseMove { x, y } => {
                if !self.mouse_locked {
                    return
                }

                let mouse_window_pos = IVec2::new(x, y);
                let mouse_delta = self.mouse.get_mouse_delta(mouse_window_pos);
                if mouse_delta == IVec2::ZERO {
                    return
                }
                self.apply_mouse_delta(mouse_delta);
                true
            }
            _ => return,
        };

        if state_dirtied {
            self.dirty = true;
        }
    }

    fn handle_key(&mut self, key: KeyCode, pressed: bool) -> bool {
        match key {
            KeyCode::W => {
                self.state.forward = pressed;
            }
            KeyCode::A => {
                self.state.left = pressed;
            }
            KeyCode::S => {
                self.state.backward = pressed;
            }
            KeyCode::D => {
                self.state.right = pressed;
            }
            KeyCode::Space => {
                self.state.shoot = pressed;
            }
            KeyCode::LShift | KeyCode::RShift => {
                self.slow_look = pressed;
                return false;
            }
            KeyCode::Escape => {
                self.mouse_locked = false;
                return false;
            }
            _ => return false,
        }

        true
    }

    fn handle_click(&mut self, button: MouseButton, pressed: bool) -> bool {
        match button {
            MouseButton::Left => {
                self.mouse_locked = true;
                self.state.shoot = pressed;
            }
            _ => return false,
        }
        true
    }

    fn apply_mouse_delta(&mut self, mouse_delta: IVec2) {
        let delta = mouse_delta.as_vec2() * MOUSE_SENSITIVITY;
        self.apply_look_delta(delta);
    }

    fn apply_look_delta(&mut self, delta: Vec2) {
        self.state.look_angle = (self.state.look_angle - delta.x)
            .rem_euclid(PI * 2.0);
        self.up_down_angle = (self.up_down_angle + delta.y)
            .clamp(-UP_DOWN_ANGLE_CLAMP, UP_DOWN_ANGLE_CLAMP);
    }
}


/// Used to track mouse movement without being constrained to the window.
/// Necessary because notan doesn't support getting raw mouse movement yet.
#[cfg(feature = "mouse-look")]
struct MouseController {
    center: IVec2,
    enigo: enigo::Enigo,
}

#[cfg(feature = "mouse-look")]
impl MouseController {
    fn new(app: &mut App) -> Self {
        assert!(app.window().is_fullscreen());
        let (window_pos, window_size) = Self::sizes(app);
        let center = window_pos + window_size / 2;

        Self {
            center,
            enigo: enigo::Enigo::new(),
        }
    }

    fn get_mouse_delta(&mut self, mouse_pos: IVec2) -> IVec2 {
        use enigo::MouseControllable;
        let mouse_delta = self.center - mouse_pos - IVec2::new(1, 1);
        self.enigo.mouse_move_to(self.center.x, self.center.y);
        mouse_delta
    }

    fn sizes(app: &mut App) -> (IVec2, IVec2) {
        let (x, y) = app.window().position();
        let window_pos = IVec2::new(x, y);
        let (width, height) = app.window().size();
        let window_size = IVec2::new(width, height);

        (window_pos, window_size)
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}