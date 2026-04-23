use crate::level::Level;
use macroquad::prelude::*;
use std::fmt;

impl fmt::Debug for GameCamera {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GameCamera")
            .field("cam", &self.cam)
            .field("smoothing", &self.smoothing)
            .field("current_target", &self.current_target)
            .finish()
    }
}

pub struct GameCamera {
    pub cam: Camera2D,
    pub smoothing: f32,
    pub current_target: Vec2,
}

impl GameCamera {
    pub fn new() -> Self {
        Self {
            cam: Camera2D::default(),
            smoothing: 0.15,
            current_target: vec2(0.0, 0.0),
        }
    }

    pub fn update(&mut self, player_pos: Vec2, player_size: Vec2, level: &Level) {
        let view_width = screen_width() / 2.0;
        let view_height = screen_height() / 2.0;

        let target_x = player_pos.x + player_size.x / 2.0;
        let target_y = player_pos.y + player_size.y / 2.0;

        self.current_target.x += (target_x - self.current_target.x) * self.smoothing;
        self.current_target.y += (target_y - self.current_target.y) * self.smoothing;

        let half_vw = view_width / 2.0;
        let half_vh = view_height / 2.0;

        let clamped_x = if level.pixel_width() < view_width {
            level.pixel_width() / 2.0 // Centrar en X
        } else {
            self.current_target
                .x
                .clamp(half_vw, level.pixel_width() - half_vw)
        };

        let clamped_y = if level.pixel_height() < view_height {
            level.pixel_height() / 2.0 // Centrar en Y
        } else {
            self.current_target
                .y
                .clamp(half_vh, level.pixel_height() - half_vh)
        };

        self.cam = Camera2D {
            target: vec2(clamped_x, clamped_y),
            zoom: vec2(2.0 / view_width, 2.0 / view_height),
            ..Default::default()
        };
    }

    pub fn apply(&self) {
        set_camera(&self.cam);
    }
}
