use application::Application;

use utils::Random;

mod utils;
mod network;
mod game;
mod application;


#[macroquad::main("Bored")]
async fn main() {    
    Random::seed();
    let mut app = Application::default();
    app.run().await;
    println!("\nFinished !");
}
