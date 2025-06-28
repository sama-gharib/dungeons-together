use std::collections::VecDeque;

use application::Application;

use macroquad::prelude::*;
use utils::{Drawable, Random};

mod utils;
mod network;
mod game;
mod application;

use game::map::{ Map, Chunk };

#[macroquad::main("Bored")]
async fn main() {    
    Random::seed();
    let mut app = Application::default();
    app.run().await;
    println!("\nFinished !");
}
