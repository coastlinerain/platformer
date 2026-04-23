use macroquad::prelude::*;

use crate::config::TILE_SIZE;

#[derive(Debug)]
pub struct Level {
    grid: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Level {
    pub fn new(data: Vec<Vec<u8>>) -> Self {
        let height = data.len();
        let width = if height > 0 { data[0].len() } else { 0 };
        Self {
            grid: data,
            width,
            height,
        }
    }

    pub fn pixel_width(&self) -> f32 {
        self.width as f32 * crate::config::TILE_SIZE
    }
    pub fn pixel_height(&self) -> f32 {
        self.height as f32 * crate::config::TILE_SIZE
    }

    fn solid_point(&self, x: f32, y: f32) -> bool {
        let gx = (x / TILE_SIZE) as usize;
        let gy = (y / TILE_SIZE) as usize;
        if gx < self.width && gy < self.height {
            return self.grid[gy][gx] == 1;
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

    pub fn draw(&self) {
        for (y, fila) in self.grid.iter().enumerate() {
            for (x, &tile) in fila.iter().enumerate() {
                if tile == 1 {
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
