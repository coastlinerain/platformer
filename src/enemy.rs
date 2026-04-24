use crate::config::*;
use crate::level::Level;
use crate::traits::{Colisionable, Entity};
use macroquad::prelude::*;
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum EnemyState {
    Idle,
    WalkingLeft,
    WalkingRight,
    Jumping,
}

impl Colisionable for Enemy {
    fn get_rect(&self) -> Rect {
        Rect::new(self.pos.x, self.pos.y, self.w, self.h)
    }
}
#[derive(Debug)]
pub struct Enemy {
    pub pos: Vec2,
    pub vel: Vec2,
    pub w: f32,
    pub h: f32,
    pub grounded: bool,
    pub state: EnemyState,
    pub timer: f32,
    pub alive: bool,
}

impl Entity for Enemy {
    fn update(&mut self, dt: f32, nivel: &Level) {
        self.timer += dt;

        if self.timer >= 0.1 {
            self.set_rand_state();
            self.timer = 0.0;
        }

        match self.state {
            EnemyState::Idle => self.vel.x = 0.0,
            EnemyState::WalkingLeft => self.vel.x = -VELOCIDAD * dt,
            EnemyState::WalkingRight => self.vel.x = VELOCIDAD * dt,
            EnemyState::Jumping => {
                if self.grounded {
                    self.vel.y = SALTO_FUERZA * dt;
                    self.grounded = false;
                }
            }
        }

        self.apply_physics(dt, nivel);
    }

    fn draw(&self) {
        draw_rectangle(self.pos.x, self.pos.y, self.w, self.h, BLUE);
    }
}

impl Enemy {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos: pos,
            vel: vec2(0.0, 0.0),
            w: 30.0,
            h: 38.0,
            grounded: false,
            state: EnemyState::Idle,
            timer: 0.0,
            alive: true,
        }
    }
    pub fn die(&mut self) {
        self.alive = false;
    }
    pub fn set_rand_state(&mut self) {
        let r = macroquad::rand::gen_range(0, 4);
        self.state = match r {
            0 => EnemyState::Idle,
            1 => EnemyState::WalkingLeft,
            2 => EnemyState::WalkingRight,
            _ => EnemyState::Jumping,
        };
    }
    fn apply_physics(&mut self, dt: f32, nivel: &crate::level::Level) {
        self.vel.y += GRAVEDAD * dt;

        // Eje X
        self.pos.x += self.vel.x;
        if nivel.map_colision(self) {
            self.pos.x -= self.vel.x;
        }

        // Eje Y
        self.pos.y += self.vel.y;
        if nivel.map_colision(self) {
            if self.vel.y > 0.0 {
                self.grounded = true;
            }
            self.pos.y -= self.vel.y;
            self.vel.y = 0.0;
        } else {
            self.grounded = false;
        }
    }
}
