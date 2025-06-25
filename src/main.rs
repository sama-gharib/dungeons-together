use std::collections::VecDeque;

use application::Application;

use macroquad::prelude::*;
use utils::{Drawable, Random};

mod utils;
mod network;
mod game;
mod application;

use game::map::Map;

#[macroquad::main("Bored")]
async fn main() {    
    Random::seed();
    let mut app = Application::default();
    app.run().await;
    println!("\nFinished !");
    
 
}

async fn _map_generation_demo() {
    let map = Map::generate(50, 50, 0);
    let mut frame_count: usize = 0;
    loop{
        if is_key_pressed(KeyCode::Space) {
            frame_count = 0;
        }
        
        for room in map.rooms.iter().take(frame_count*10) {
            for component in room.components.iter() {
                if let Some(component) = component {
                    component.draw();
                }
            }
        }
        
        frame_count += 1;
        next_frame().await;
    }
}