use crate::network::GamePacket;
use macroquad::math::Vec2; // Asegúrate de importar tu Enum de paquetes
use macroquad::prelude::*;

#[derive(Debug)]
pub struct Nemesis {
    pub id: u64,
    pub pos: Vec2,
    pub w: f32,
    pub h: f32,
    pub last_dir: f32,
}

impl Nemesis {
    pub fn new(id: u64, pos: Vec2) -> Self {
        Self {
            id,
            pos,
            w: 30.0,
            h: 38.0,
            last_dir: 1.0,
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.pos.x, self.pos.y, self.w, self.h, ORANGE);

        // Dibujar un pequeño indicador de hacia dónde mira
        let eye_x = if self.last_dir > 0.0 {
            self.pos.x + 20.0
        } else {
            self.pos.x + 5.0
        };
        draw_rectangle(eye_x, self.pos.y + 10.0, 5.0, 5.0, WHITE);

        // Dibujar el ID sobre la cabeza del jugador
        draw_text(
            &format!("P: {}", self.id),
            self.pos.x,
            self.pos.y - 10.0,
            15.0,
            WHITE,
        );
    }
}
