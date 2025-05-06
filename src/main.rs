use std::{borrow::BorrowMut, sync::{Arc, Mutex}, thread::{self, sleep}, time::Duration};

use macroquad::prelude::*;

use network::{ client::GameClient, server::GameServer};
use utils::Random;
use network::GameAgent;

mod utils;
mod network;
mod game;

enum Mode {
    Server,
    Client
}


#[macroquad::main("Bored")]
async fn main() {
    
    Random::seed();
    
    let args: Vec<_> = std::env::args().collect();

    let mode = match args.get(1) {
       Some(argument) => match &argument[..]{
            "server" => Mode::Server,
            "client" => Mode::Client,
            e        => panic!("Unknown argument: {e}")
       },
       None           => panic!("Too few arguments.") 
    };
    
    let mut s: Arc<Mutex<dyn GameAgent + Send>> = match mode {
        Mode::Server => Arc::new(Mutex::new(GameServer::new("0.0.0.0:53000").unwrap())),
        Mode::Client => Arc::new(Mutex::new(GameClient::new("localhost:53000").unwrap()))
    };
    
    let mut my_s = Arc::clone(&s);
    
    let backend_thread = thread::spawn( move || {
        
        let mut s = Arc::clone(&s);
        
        loop {
            s.borrow_mut().lock().unwrap().update();
            
            sleep(Duration::from_millis(8));
        }
    });
    
    loop {            
        clear_background(BLACK);
        draw_text("Work in progress.", 100., 100., 32., WHITE);
        
        if let Ok(mut my_s) = my_s.borrow_mut().lock() {
            my_s.handle_events();
            my_s.draw();
        }
        
        next_frame().await;
    }
}

