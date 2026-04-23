use crate::config::BULLET_SPEED;
use crate::enemy::Enemy;
use crate::level::Level;
use crate::traits::{Colisionable, Entity, detect_collision};
use macroquad::prelude::*;

#[derive(Debug)]
pub struct Bullet {
    pub pos: Vec2,
    pub vel: Vec2,
    pub active: bool,
}

impl Colisionable for Bullet {
    fn get_rect(&self) -> Rect {
        Rect::new(self.pos.x, self.pos.y, 5.0, 5.0)
    }
}

impl Entity for Bullet {
    fn update(&mut self, dt: f32, nivel: &Level) {
        self.pos += self.vel * dt;
        if nivel.colision(self.pos.x, self.pos.y, 5.0, 5.0) {
            self.active = false;
        }
    }
    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 5.0, YELLOW);
    }
}

impl Bullet {
    pub fn new(pos: Vec2, dir_x: f32) -> Self {
        Self {
            pos,
            vel: vec2(dir_x * BULLET_SPEED, 0.0),
            active: true,
        }
    }
    pub fn check_collision(&self, enemy: &Enemy) -> bool {
        detect_collision(self, enemy)
    }
}
