mod bullet;
mod camera;
mod config;
mod enemy;
mod level;
mod maps;
mod nemesis;
mod network;
mod player;
mod traits;
mod world;

use crate::maps::Map;
use macroquad::prelude::*;
use world::World;

#[macroquad::main("Rustvania")]
async fn main() {
    let world_data = Map::new();
    let network_client = crate::network::NetworkClient::new("127.0.0.1:12345");
    let mut game_world = World::new(world_data, network_client);

    loop {
        if is_key_pressed(KeyCode::P) {
            println!("{:#?}", game_world);
        }
        clear_background(BLACK);
        game_world.update(get_frame_time());
        game_world.draw();

        next_frame().await
    }
}
