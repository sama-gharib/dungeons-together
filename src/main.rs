use macroquad::prelude::*;

mod network;

#[macroquad::main("Bored")]
async fn main() {
    loop {
        
        clear_background(BLACK);
        draw_text("Work in progress.", 100., 100., 32., WHITE);
        
        next_frame().await;
    }
}
