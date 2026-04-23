mod bullet;
mod camera;
mod config;
mod enemy;
mod level;
mod maps;
mod player;
mod traits;
mod world;

use crate::maps::get_world_matrix;
use macroquad::prelude::*;
use world::World;

#[macroquad::main("Rustvania")]
async fn main() {
    let world_data = get_world_matrix();
    let mut game_world = World::new(world_data);

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
