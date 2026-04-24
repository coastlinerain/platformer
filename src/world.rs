// src/world.rs
use crate::camera::GameCamera;
use crate::config::TILE_SIZE;
use crate::enemy::Enemy;
use crate::level::Level;
use crate::player::Player;
use crate::traits::Entity;
use macroquad::prelude::*;
use std::fmt;

pub struct World {
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub levels_matrix: Vec<Vec<Level>>, // Matriz de niveles
    pub current_coords: (usize, usize),
    pub camera: GameCamera,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileType {
    Empty,
    Wall,
    EnemySpawn,
    BossSpawn,
    SavePoint,
}

impl From<u8> for TileType {
    fn from(value: u8) -> Self {
        match value {
            1 => TileType::Wall,
            2 => TileType::EnemySpawn,
            3 => TileType::BossSpawn,
            _ => TileType::Empty, // El 0 y cualquier otro caen aquí
        }
    }
}

impl fmt::Debug for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("World")
            .field("player", &self.player)
            .field("enemies", &self.enemies)
            .field("levels_matrix", &self.levels_matrix.len())
            .field("current_coords", &self.current_coords)
            .field("camera", &self.camera)
            .finish()
    }
}

impl World {
    pub fn new(matrix_data: Vec<Vec<Vec<Vec<TileType>>>>) -> Self {
        let levels = matrix_data
            .into_iter()
            .map(|row| row.into_iter().map(Level::new).collect())
            .collect();

        Self {
            player: Player::new(),
            enemies: vec![],
            levels_matrix: levels,
            current_coords: (0, 0),
            camera: GameCamera::new(),
        }
    }
    fn load_current_level(&mut self) {
        self.enemies.clear();
        self.player.bullets.clear();

        let level = &self.levels_matrix[self.current_coords.1][self.current_coords.0];

        for (y, row) in level.grid.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                if tile == TileType::EnemySpawn {
                    let pos = vec2(
                        x as f32 * crate::config::TILE_SIZE,
                        y as f32 * crate::config::TILE_SIZE,
                    );
                    self.enemies.push(Enemy::new(pos));
                }
            }
        }

        println!(
            "Nivel {},{} cargado con {} enemigos",
            self.current_coords.0,
            self.current_coords.1,
            self.enemies.len()
        );
    }
    fn check_level_transitions(&mut self) {
        let (cx, cy) = self.current_coords;
        let level_w = self.levels_matrix[cy][cx].pixel_width();
        let level_h = self.levels_matrix[cy][cx].pixel_height();
        let p = &mut self.player;

        let mut changed = false;

        // --- SALIDA DERECHA ---
        // println!(
        //     "Intentando salir por la derecha... p.pos.x: {}, level_w: {}",
        //     p.pos.x, level_w
        // );
        // println!("Player position: ({}, {})", p.pos.x, p.pos.y);
        // println!("Current level size: ({} x {})", level_w, level_h);
        if p.pos.x + p.w > level_w - 5.0 {
            // Un pequeño margen de 5px
            if cx < self.levels_matrix[0].len() - 1 {
                self.current_coords.0 += 1;
                p.pos.x = 10.0;
                changed = true;
            }
        }
        // --- SALIDA IZQUIERDA ---
        else if p.pos.x < -5.0 {
            if cx > 0 {
                self.current_coords.0 -= 1;
                p.pos.x = self.levels_matrix[cy][cx - 1].pixel_width() - p.w - 10.0;
                changed = true;
            }
        }

        // --- SALIDA ABAJO ---
        if p.pos.y + p.h > level_h - 5.0 {
            if cy < self.levels_matrix.len() - 1 {
                self.current_coords.1 += 1;
                p.pos.y = 10.0;
                changed = true;
            }
        }
        // --- SALIDA ARRIBA ---
        else if p.pos.y + p.h < 5.0 {
            if cy > 0 {
                self.current_coords.1 -= 1;
                p.pos.y = self.levels_matrix[cy - 1][cx].pixel_height() - p.h - 10.0;
                changed = true;
            }
        }

        if changed {
            println!(
                "Cambiado a nivel: {},{}",
                self.current_coords.0, self.current_coords.1
            );
            self.load_current_level();
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.check_level_transitions();

        let level = &self.levels_matrix[self.current_coords.1][self.current_coords.0];

        self.player.update(dt, &level);

        for enemy in &mut self.enemies {
            enemy.update(dt, &level);
        }

        for bullet in &mut self.player.bullets {
            for enemy in &mut self.enemies {
                if bullet.check_collision(enemy) {
                    bullet.active = false;
                    enemy.die();
                    println!("¡Enemigo alcanzado!");
                }
            }
        }
        self.enemies.retain(|e| e.alive);
        self.camera
            .update(self.player.pos, vec2(self.player.w, self.player.h), level);
    }

    pub fn spawn_entities(&mut self, level: &Level) {
        for (y, row) in level.grid.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let pos = vec2(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);

                match tile {
                    TileType::EnemySpawn => self.enemies.push(Enemy::new(pos)),
                    _ => {}
                }
            }
        }
    }

    pub fn draw(&self) {
        self.camera.apply();
        let level = &self.levels_matrix[self.current_coords.1][self.current_coords.0];
        level.draw();
        self.player.draw();
        for bullet in &self.player.bullets {
            bullet.draw();
        }
        for enemy in &self.enemies {
            enemy.draw();
        }
        set_default_camera();
        //HUD
        draw_text(
            &format!("Level: {},{}", self.current_coords.0, self.current_coords.1),
            10.0,
            20.0,
            20.0,
            WHITE,
        );
    }
}
