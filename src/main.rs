use macroquad::prelude::*;
use network::server::GameServer;

mod network;

#[macroquad::main("Bored")]
async fn main() {
    // Testing server
    
    let mut s = GameServer::new("localhost:53000").unwrap();
    
    loop {
        
        s.default_behaviour();
        
        clear_background(BLACK);
        draw_text("Work in progress.", 100., 100., 32., WHITE);
        
        next_frame().await;
    }
}
