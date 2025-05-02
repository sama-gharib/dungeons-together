use macroquad::prelude::*;

use network::{client::GameClient, server::GameServer};
use utils::DefaultBehaviour;

mod utils;
mod network;

enum Mode {
    Server,
    Client
}

#[macroquad::main("Bored")]
async fn main() {
    // Testing server
    
    let mut args: Vec<_> = std::env::args().collect();

    let mode = match args.get(1) {
       Some(argument) => match &argument[..]{
            "server" => Mode::Server,
            "client" => Mode::Client,
            e        => panic!("Unknown argument: {e}")
       },
       None           => panic!("Too few arguments.") 
    };
    
    let mut s: Box<dyn DefaultBehaviour> = match mode {
        Mode::Server => Box::new(GameServer::new("localhost:53000").unwrap()),
        Mode::Client => Box::new(GameClient::new("localhost:53000").unwrap())
    };
    
    loop {
        
        s.default_behaviour();
        
        clear_background(BLACK);
        draw_text("Work in progress.", 100., 100., 32., WHITE);
        
        next_frame().await;
    }
}
