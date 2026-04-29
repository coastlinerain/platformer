use crate::entities::bullet::Bullet;
use crate::entities::nemesis::Nemesis;
use crate::network::client::{GamePacket, NetworkClient};
use crate::world::world::World;
use laminar::SocketEvent;
use macroquad::prelude::*;

pub fn update_network(world: &mut World) {
    // 1. RECIBIR: Escuchar qué dice el servidor
    while let Ok(event) = world.network.receiver.try_recv() {
        if let SocketEvent::Packet(packet) = event {
            if let Ok(decoded) = postcard::from_bytes::<GamePacket>(packet.payload()) {
                handle_packet(world, decoded);
            }
        }
    }

    // 2. ENVIAR: Solo si el servidor ya nos dio un ID
    if let Some(id) = world.id {
        let pos_msg = GamePacket::PlayerPos {
            id,
            x: world.player.pos.x,
            y: world.player.pos.y,
            dir: world.player.dir,
            level_x: world.current_coords.0 as u8,
            level_y: world.current_coords.1 as u8,
        };
        let bytes = postcard::to_allocvec(&pos_msg).unwrap();
        world
            .network
            .sender
            .send(laminar::Packet::unreliable(
                world.network.server_addr,
                bytes,
            ))
            .unwrap();

        if is_key_pressed(KeyCode::Z) {
            let shoot_msg = GamePacket::Action {
                id: world.id.unwrap(),
                kind: "shoot".to_string(),
                dir: world.player.dir as f32,
            };
            let bytes = postcard::to_allocvec(&shoot_msg).unwrap();
            world
                .network
                .sender
                .send(laminar::Packet::unreliable(
                    world.network.server_addr,
                    bytes,
                ))
                .unwrap();
        }
    }
}
fn handle_packet(world: &mut World, decoded: GamePacket) {
    match decoded {
        GamePacket::JoinResponse { assigned_id } => {
            world.id = Some(assigned_id);
            world.player.id = assigned_id;
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
            if Some(id) != world.id {
                if let Some(nemesis) = world.other_players.get_mut(&id) {
                    nemesis.pos = vec2(x, y);
                    nemesis.last_dir = dir;
                    nemesis.coords = (level_x as usize, level_y as usize)
                } else {
                    println!("Creamos jugador {}", id);
                    let mut new_nemesis = Nemesis::new(id, vec2(x, y));
                    new_nemesis.last_dir = dir;
                    new_nemesis.coords = (level_x as usize, level_y as usize);

                    // Insertamos usando 'id' como clave y el objeto como valor
                    world.other_players.insert(id, new_nemesis);
                }
            }
        }
        GamePacket::Action { id, kind, dir } => {
            if kind.to_string() == "shoot" {
                if let Some(enemy) = world.other_players.get(&id) {
                    println!("El jugador {} disparó en {:?}", id, enemy.pos);

                    let spawn_center =
                        vec2(enemy.pos.x + (enemy.w / 2.0), enemy.pos.y + (enemy.h / 2.0));

                    if let Some(owner) = world.other_players.get(&id) {
                        if owner.coords == world.current_coords {
                            println!("SAME room");

                            world.enemy_bullets.push(Bullet::new(
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
            if let Some(_removed_enemy) = world.other_players.remove(&id) {
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
pub fn send_join_req(network: &NetworkClient) {
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
}
