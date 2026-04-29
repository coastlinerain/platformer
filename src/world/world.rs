use crate::camera::GameCamera;
use crate::config::TILE_SIZE;
use crate::entities::bullet::Bullet;
use crate::entities::enemy::Enemy;
use crate::entities::nemesis::Nemesis;
use crate::entities::player::Player;
use crate::level::Level;
use crate::maps::Map;
use crate::network::GamePacket;
use crate::traits::Entity;
use laminar::SocketEvent;
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
    pub network: crate::network::NetworkClient,
    pub other_players: HashMap<u8, Nemesis>,
    pub enemy_bullets: Vec<Bullet>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileType {
    Empty,
    Wall,
    EnemySpawn,
    BossSpawn,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EntityType {
    Empty,
    EnemySpawn,
    BossSpawn,
    PotionSpawn,
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

impl From<u8> for EntityType {
    fn from(value: u8) -> Self {
        match value {
            1 => EntityType::EnemySpawn,
            2 => EntityType::BossSpawn,
            3 => EntityType::PotionSpawn,
            _ => EntityType::Empty, // El 0 y cualquier otro caen aquí
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
            .field("other_players", &self.other_players)
            .finish()
    }
}

impl World {
    pub fn new(matrix_data: Map, network: crate::network::NetworkClient) -> Self {
        let join_msg = GamePacket::JoinRequest;
        let bytes = postcard::to_allocvec(&join_msg).unwrap();

        network
            .sender
            .send(laminar::Packet::reliable_ordered(
                network.server_addr,
                bytes,
                None,
            ))
            .unwrap();

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
    fn load_current_level(&mut self) {
        self.enemies.clear();
        self.player.bullets.clear();

        let level = &self.levels_matrix[self.current_coords.1][self.current_coords.0];

        for (y, row) in level.data.iter().enumerate() {
            for (x, &data) in row.iter().enumerate() {
                let pos = vec2(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);

                match data {
                    EntityType::EnemySpawn => self.enemies.push(Enemy::new(pos, BLUE)),
                    EntityType::PotionSpawn => self.enemies.push(Enemy::new(pos, GREEN)),
                    _ => {}
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
        self.update_enemy_bullets_position(dt);
        self.check_player_death();
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
        self.update_network();
        self.enemies.retain(|e| e.alive);
    }
    fn update_enemy_bullets_position(&mut self, dt: f32) {
        let current_room = self.current_coords;
        let level = &self.levels_matrix[current_room.1][current_room.0];

        self.enemy_bullets.retain_mut(|bullet| {
            if let Some(owner) = self.other_players.get(&bullet.owner_id) {
                bullet.update(dt, level);
                bullet.active
            } else {
                false
            }
        });
    }
    fn check_player_death(&mut self) {
        let mut died = false;

        for bullet in &mut self.enemy_bullets {
            if bullet.active && bullet.check_collision(&self.player) {
                bullet.active = false;
                died = true;
                break;
            }
        }

        if died {
            self.handle_death();
        }
    }
    fn handle_death(&mut self) {
        if let Some(my_id) = self.id {
            let death_msg = GamePacket::Hit { entity_id: my_id }; // O GamePacket::Death
            let bytes = postcard::to_allocvec(&death_msg).unwrap();

            let _ = self.network.sender.send(laminar::Packet::reliable_ordered(
                self.network.server_addr,
                bytes,
                None,
            ));
        }

        println!("¡Has muerto! Volviendo al inicio...");
        self.current_coords = (0, 0);
        self.player.pos = vec2(100.0, 100.0);
        self.load_current_level();
    }
    fn update_network(&mut self) {
        // 1. RECIBIR: Escuchar qué dice el servidor
        while let Ok(event) = self.network.receiver.try_recv() {
            if let SocketEvent::Packet(packet) = event {
                if let Ok(decoded) = postcard::from_bytes::<GamePacket>(packet.payload()) {
                    self.handle_packet(decoded);
                }
            }
        }

        // 2. ENVIAR: Solo si el servidor ya nos dio un ID
        if let Some(id) = self.id {
            let pos_msg = GamePacket::PlayerPos {
                id,
                x: self.player.pos.x,
                y: self.player.pos.y,
                dir: self.player.dir,
                level_x: self.current_coords.0 as u8,
                level_y: self.current_coords.1 as u8,
            };
            let bytes = postcard::to_allocvec(&pos_msg).unwrap();
            self.network
                .sender
                .send(laminar::Packet::unreliable(self.network.server_addr, bytes))
                .unwrap();

            if is_key_pressed(KeyCode::Z) {
                let shoot_msg = GamePacket::Action {
                    id: self.id.unwrap(),
                    kind: "shoot".to_string(),
                    dir: self.player.dir as f32,
                };
                let bytes = postcard::to_allocvec(&shoot_msg).unwrap();
                self.network
                    .sender
                    .send(laminar::Packet::unreliable(self.network.server_addr, bytes))
                    .unwrap();
            }
        }
    }
    fn handle_packet(&mut self, decoded: GamePacket) {
        match decoded {
            GamePacket::JoinResponse { assigned_id } => {
                self.id = Some(assigned_id);
                self.player.id = assigned_id;
                println!("¡Conectado! Mi ID es {}", assigned_id);
            }
            GamePacket::PlayerPos {
                id,
                x,
                y,
                dir,
                level_x,
                level_y,
            } => {
                if Some(id) != self.id {
                    if let Some(nemesis) = self.other_players.get_mut(&id) {
                        nemesis.pos = vec2(x, y);
                        nemesis.last_dir = dir;
                        nemesis.coords = (level_x as usize, level_y as usize)
                    } else {
                        println!("Creamos jugador {}", id);
                        let mut new_nemesis = Nemesis::new(id, vec2(x, y));
                        new_nemesis.last_dir = dir;
                        new_nemesis.coords = (level_x as usize, level_y as usize);

                        // Insertamos usando 'id' como clave y el objeto como valor
                        self.other_players.insert(id, new_nemesis);
                    }
                }
            }
            GamePacket::Action { id, kind, dir } => {
                if kind.to_string() == "shoot" {
                    if let Some(enemy) = self.other_players.get(&id) {
                        println!("El jugador {} disparó en {:?}", id, enemy.pos);

                        let spawn_center =
                            vec2(enemy.pos.x + (enemy.w / 2.0), enemy.pos.y + (enemy.h / 2.0));

                        if let Some(owner) = self.other_players.get(&id) {
                            if owner.coords == self.current_coords {
                                println!("SAME room");

                                self.enemy_bullets.push(Bullet::new(
                                    spawn_center,
                                    dir as f32,
                                    enemy.id,
                                ));
                            }
                        }
                    } else {
                        println!(
                            "DEBUG: Recibido disparo de ID {} pero no lo tengo en mi lista!",
                            id
                        );
                    }
                }
            }
            GamePacket::Leave { id } => {
                if let Some(_removed_enemy) = self.other_players.remove(&id) {
                    println!("El jugador {} se ha ido. Eliminando del mundo.", id);
                } else {
                    println!(
                        "DEBUG: Intento de borrar ID {} que no existía en el cliente.",
                        id
                    );
                }
            }
            _ => {}
        }
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
