use crate::config::*;
use crate::entities::bullet::Bullet;
use crate::level::Level;
use crate::traits::{Colisionable, Entity};
use macroquad::prelude::*;

#[derive(Debug)]
pub struct Player {
    pub pos: Vec2,
    pub vel: Vec2,
    pub w: f32,
    pub h: f32,
    pub grounded: bool,
    pub dir: f32,
    pub bullets: Vec<Bullet>,
}
impl Colisionable for Player {
    fn get_rect(&self) -> Rect {
        Rect::new(self.pos.x, self.pos.y, self.w, self.h)
    }
}
impl Entity for Player {
    fn update(&mut self, dt: f32, nivel: &Level) {
        self.vel.x = 0.0;
        if is_key_down(KeyCode::Right) {
            self.vel.x = VELOCIDAD * dt;
        }
        if is_key_down(KeyCode::Left) {
            self.vel.x = -VELOCIDAD * dt;
        }

        if is_key_pressed(KeyCode::Space) && self.grounded {
            self.vel.y = SALTO_FUERZA * dt;
        }

        //direccion
        if is_key_down(KeyCode::Right) {
            self.dir = 1.0;
        }
        if is_key_down(KeyCode::Left) {
            self.dir = -1.0;
        }
        //shoot
        if is_key_pressed(KeyCode::Z) {
            self.bullets.push(Bullet::new(
                self.pos + vec2(self.w / 2.0, self.h / 2.0),
                self.dir as f32,
            ));
        }
        // update bullets
        self.bullets.iter_mut().for_each(|b| b.update(dt, &nivel));

        self.vel.y += GRAVEDAD * dt;

        // Eje X
        self.pos.x += self.vel.x;
        if nivel.colision(self.pos.x, self.pos.y, self.w, self.h) {
            self.pos.x -= self.vel.x;
        }

        // Eje Y
        self.pos.y += self.vel.y;
        if nivel.colision(self.pos.x, self.pos.y, self.w, self.h) {
            if self.vel.y > 0.0 {}
            self.pos.y -= self.vel.y;
            self.vel.y = 0.0;
        } else {
        }

        self.grounded = nivel.grounded(self.get_rect());

        self.bullets.retain(|b| b.active);
    }

    fn draw(&self) {
        draw_rectangle(self.pos.x, self.pos.y, self.w, self.h, RED);
    }
}
impl Player {
    pub fn new() -> Self {
        Self {
            pos: vec2(100.0, 100.0),
            vel: vec2(0.0, 0.0),
            w: 30.0,
            h: 38.0,
            grounded: false,
            dir: 1.0,
            bullets: Vec::new(),
        }
    }
}
