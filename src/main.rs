use application::Application;

use macroquad::window::next_frame;
use utils::{Drawable, Random};

mod utils;
mod network;
mod game;
mod application;

use game::map::Map;

#[macroquad::main("Bored")]
async fn main() {    
    Random::seed();
    /*let mut app = Application::default();
    app.run().await;
    println!("\nFinished !");*/
    
    let map = Map::generate(50, 50);
    let mut frame_count: usize = 0;
    loop{
        for room in map.rooms.iter().take(frame_count*20) {
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
