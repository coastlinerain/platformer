use crate::camera::GameCamera;
use crate::config::TILE_SIZE;
use crate::entities::bullet::Bullet;
use crate::entities::enemy::Enemy;
use crate::entities::nemesis::Nemesis;
use crate::entities::player::Player;
use crate::level::Level;
use crate::maps::Map;
use crate::world::lib::{
    check_level_transitions, check_player_death, update_enemy_bullets_position,
};

use crate::network::client::NetworkClient;
use crate::network::world::{send_join_req, update_network};
use crate::traits::Entity;
use crate::types::EntityType;
use macroquad::prelude::*;
use std::collections::HashMap;
use std::fmt;

pub struct World {
    pub id: Option<u8>,
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub levels_matrix: Vec<Vec<Level>>, // Matriz de niveles
    pub current_coords: (usize, usize),
    pub camera: GameCamera,
    pub network: crate::network::client::NetworkClient,
    pub other_players: HashMap<u8, Nemesis>,
    pub enemy_bullets: Vec<Bullet>,
}

impl fmt::Debug for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("World")
            .field("player", &self.player)
            .field("enemies", &self.enemies)
            .field("levels_matrix", &self.levels_matrix.len())
            .field("current_coords", &self.current_coords)
            .field("camera", &self.camera)
            .field("other_players", &self.other_players)
            .finish()
    }
}

impl World {
    pub fn new(matrix_data: Map, network: NetworkClient) -> Self {
        send_join_req(&network);

        let levels: Vec<Vec<Level>> = matrix_data
            .grids
            .into_iter()
            .zip(matrix_data.data.into_iter())
            .map(|(grid_row, data_row)| {
                grid_row
                    .into_iter()
                    .zip(data_row.into_iter())
                    .map(|(grid, data)| Level::new(grid, data))
                    .collect()
            })
            .collect();

        Self {
            id: None,
            player: Player::new(0),
            enemies: vec![],
            levels_matrix: levels,
            current_coords: (0, 0),
            camera: GameCamera::new(),
            network: network,
            other_players: HashMap::new(),
            enemy_bullets: Vec::new(),
        }
    }
    pub fn update(&mut self, dt: f32) {
        check_level_transitions(self);
        update_enemy_bullets_position(self, dt);
        check_player_death(self);
        {
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
            self.camera
                .update(self.player.pos, vec2(self.player.w, self.player.h), &level);
        }
        update_network(self);
        self.enemies.retain(|e| e.alive);
    }

    pub fn spawn_entities(&mut self, level: &Level) {
        for (y, row) in level.data.iter().enumerate() {
            for (x, data) in row.iter().enumerate() {
                let pos = vec2(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);

                match data {
                    EntityType::EnemySpawn => self.enemies.push(Enemy::new(pos, BLUE)),
                    EntityType::PotionSpawn => self.enemies.push(Enemy::new(pos, GREEN)),
                    _ => {}
                }
            }
        }
    }

    pub fn draw(&self) {
        self.camera.apply();

        let level = &self.levels_matrix[self.current_coords.1][self.current_coords.0];
        level.draw();

        // --- RENDERIZAR OTROS JUGADORES ---
        for (id, enemy) in &self.other_players {
            if enemy.coords == self.current_coords {
                enemy.draw();
            }
        }

        self.player.draw();

        for bullet in &self.player.bullets {
            bullet.draw();
        }

        for bullet in &self.enemy_bullets {
            bullet.draw();
        }

        for enemy in &self.enemies {
            enemy.draw();
        }

        set_default_camera();

        // --- HUD (Coordenadas de pantalla fija) ---
        draw_text(
            &format!("Level: {},{}", self.current_coords.0, self.current_coords.1),
            10.0,
            20.0,
            20.0,
            WHITE,
        );

        if let Some(my_id) = self.id {
            draw_text(&format!("My ID: {}", my_id), 10.0, 40.0, 20.0, GREEN);
        } else {
            draw_text("Connecting...", 10.0, 40.0, 20.0, RED);
        }
    }
}
