use macroquad::prelude::*;

use crate::{
    config::TILE_SIZE,
    traits::Colisionable,
    world::{EntityType, TileType},
};

#[derive(Debug)]
pub struct Level {
    pub grid: Vec<Vec<TileType>>,
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<EntityType>>,
}

impl Level {
    pub fn new(grid: Vec<Vec<TileType>>, data: Vec<Vec<EntityType>>) -> Self {
        let height = grid.len();
        let width = if height > 0 { grid[0].len() } else { 0 };
        Self {
            grid: grid,
            width,
            height,
            data: data,
        }
    }

    pub fn pixel_width(&self) -> f32 {
        self.width as f32 * crate::config::TILE_SIZE
    }
    pub fn pixel_height(&self) -> f32 {
        self.height as f32 * crate::config::TILE_SIZE
    }

    pub fn map_colision<T: Colisionable>(&self, objeto: &T) -> bool {
        let r = objeto.get_rect();
        self.colision(r.x, r.y, r.w, r.h)
    }

    pub fn try_move(&self, rect: Rect, vel: Vec2) -> Vec2 {
        let mut nueva_pos = rect.point();

        // --- Resolución Eje X ---
        let rect_x = Rect::new(nueva_pos.x + vel.x, nueva_pos.y, rect.w, rect.h);
        if !self.solid_in_rect(rect_x) {
            nueva_pos.x += vel.x;
        } else {
            // Ajuste fino: Pegar al borde del tile
            if vel.x > 0.0 {
                nueva_pos.x =
                    ((nueva_pos.x + rect.w + vel.x) / TILE_SIZE).floor() * TILE_SIZE - rect.w;
            } else if vel.x < 0.0 {
                nueva_pos.x = ((nueva_pos.x + vel.x) / TILE_SIZE).floor() * TILE_SIZE + TILE_SIZE;
            }
        }

        // --- Resolución Eje Y ---
        let rect_y = Rect::new(nueva_pos.x, nueva_pos.y + vel.y, rect.w, rect.h);
        if !self.solid_in_rect(rect_y) {
            nueva_pos.y += vel.y;
        } else {
            if vel.y > 0.0 {
                nueva_pos.y =
                    ((nueva_pos.y + rect.h + vel.y) / TILE_SIZE).floor() * TILE_SIZE - rect.h;
            } else if vel.y < 0.0 {
                // Saltando
                nueva_pos.y = ((nueva_pos.y + vel.y) / TILE_SIZE).floor() * TILE_SIZE + TILE_SIZE;
            }
        }

        nueva_pos
    }

    fn solid_point(&self, x: f32, y: f32) -> bool {
        let gx = (x / TILE_SIZE) as usize;
        let gy = (y / TILE_SIZE) as usize;
        if gx < self.width && gy < self.height {
            return self.grid[gy][gx] == TileType::Wall;
        }
        true
    }

    pub fn colision(&self, rect_x: f32, rect_y: f32, w: f32, h: f32) -> bool {
        let puntos = [
            (rect_x + 2.0, rect_y),
            (rect_x + w - 2.0, rect_y),
            (rect_x + 2.0, rect_y + h),
            (rect_x + w - 2.0, rect_y + h),
            (rect_x, rect_y + h / 2.0),
            (rect_x + w, rect_y + h / 2.0),
        ];
        puntos.iter().any(|&(px, py)| self.solid_point(px, py))
    }

    pub fn grounded(&self, rect: Rect) -> bool {
        let chequeo_suelo = Rect::new(rect.x, rect.y + 1.0, rect.w, rect.h);
        self.solid_in_rect(chequeo_suelo)
    }

    pub fn solid_in_rect(&self, r: Rect) -> bool {
        let x_start = (r.x / TILE_SIZE).floor() as i32;
        let x_end = ((r.x + r.w) / TILE_SIZE).floor() as i32;
        let y_start = (r.y / TILE_SIZE).floor() as i32;
        let y_end = ((r.y + r.h) / TILE_SIZE).floor() as i32;

        for gy in y_start..=y_end {
            for gx in x_start..=x_end {
                if gx >= 0 && gx < self.width as i32 && gy >= 0 && gy < self.height as i32 {
                    if self.grid[gy as usize][gx as usize] == TileType::Wall {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn draw(&self) {
        for (y, fila) in self.grid.iter().enumerate() {
            for (x, &tile) in fila.iter().enumerate() {
                if tile == TileType::Wall {
                    draw_rectangle(
                        x as f32 * TILE_SIZE,
                        y as f32 * TILE_SIZE,
                        TILE_SIZE - 1.0,
                        TILE_SIZE - 1.0,
                        GRAY,
                    );
                }
            }
        }
    }
}
