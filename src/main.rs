use std::{borrow::BorrowMut, sync::{Arc, Mutex}, thread::{self, sleep}, time::Duration};

use game::Drawable;
use macroquad::prelude::*;

use network::{ client::GameClient, server::GameServer};
use utils::Random;
use network::GameAgent;
use ui::{Widget, Action};

mod utils;
mod network;
mod ui;
mod game;

enum Mode {
    Server,
    Client
}

enum AppState {
    Hosting,
    Joining,
    Finished
}

#[macroquad::main("Bored")]
async fn main() {
    
    Random::seed();
    
    let menu_button_text = Widget::default_label()
        .with_primary(BLACK)
        .with_secondary(DARKGRAY);
    
    let menu_button= Widget::default_button()
        .with_primary(DARKGRAY)
        .with_secondary(GRAY);
    
    let mut ui = Widget::default()
        .with_name("bg")
        .with_children(&mut [
            Widget::default_label()
                .with_name("Bored")
                .with_center(vec2(0.0, -0.4))
                .with_size(vec2(0.8, 0.15)),
            menu_button.clone()
                .with_id(000)
                .with_center(vec2(0.0, -0.1))
                .with_children(&mut [menu_button_text.clone().with_name("Join")]),
            menu_button.clone()
                .with_id(111)
                .with_center(vec2(0.0, 0.05))
                .with_children(&mut [menu_button_text.clone().with_name("Host")]),
            menu_button.clone()
                .with_id(999)
                .with_name("Quit button")
                .with_center(vec2(0.0, 0.2))
                .with_children(&mut [menu_button_text.clone().with_id(555).with_name("Quit")]),
        ]);

    ui.recalculate_absolutes(vec2(400.0, 300.0), vec2(800.0, 600.0));
    
    'app: loop {
        let state;
        
        'menu: loop {
            
            clear_background(GREEN);
            
            for activation in ui.get_activations() {
                println!("{:?}", activation);
                if let Action::Activate = activation.get_action() {
                    state = match activation.get_source() {
                        000 => AppState::Joining,
                        111 => AppState::Hosting,
                        _  => AppState::Finished
                    };
                    break 'menu;
                } 
            }
            
            ui.draw();
            
            next_frame().await;
        }
                
        let mode = match state  {
            AppState::Finished => break 'app,
            AppState::Hosting  => Mode::Server,
            AppState::Joining  => Mode::Client
        };
        
        default_game(mode).await;
        
    }
    
}

async fn default_game(mode: Mode) {
    
    let mut s: Arc<Mutex<dyn GameAgent + Send>> = match mode {
        Mode::Server => Arc::new(Mutex::new(GameServer::new("0.0.0.0:53000").unwrap())),
        Mode::Client => Arc::new(Mutex::new(GameClient::new("localhost:53000").unwrap()))
    };
    
    let mut should_stop = Arc::new(Mutex::new(false));
    
    let backend_thread = thread::spawn( {
        let mut should_stop: Arc<Mutex<bool>> = Arc::clone(&should_stop);
        let mut s = Arc::clone(&s);
        move || {
            loop {
                s.borrow_mut().lock().unwrap().update();
                
                sleep(Duration::from_millis(8));
                if *should_stop.borrow_mut().lock().unwrap() {
                    break;
                }
            }
        }
    });
    
    let mut ui = Widget::default_button()
        .with_center(vec2(0.4, -0.4))
        .with_size(vec2(0.15, 0.1))
        .with_children(&mut [Widget::default_label().with_name("Menu").with_primary(BLACK)]);
    ui.recalculate_absolutes(vec2(400.0, 300.0), vec2(800.0, 600.0));
    
    'game: loop {            
        clear_background(BLACK);
        draw_text("Work in progress.", 100., 100., 32., WHITE);
        
        if let Ok(mut s) = s.borrow_mut().lock() {
            s.handle_events();
            s.draw();
        }
        
        ui.draw();
        for activation in ui.get_activations() {
            if let Action::Activate = activation.get_action() {
                break 'game;
            }
        }
        
        if *should_stop.borrow_mut().lock().unwrap() {
            break;
        }
        
        next_frame().await;
    }
    
    *(should_stop.borrow_mut().lock().unwrap()) = true;
    
    if let Err(e) = backend_thread.join() {
        println!("Failed to join backend thread : {e:?}");
    }
    
}

