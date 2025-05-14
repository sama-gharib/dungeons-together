use std::{borrow::BorrowMut, sync::{Arc, Mutex}, thread::{self, sleep}, time::Duration};

use game::Drawable;
use macroquad::prelude::*;

use network::{ client::GameClient, server::GameServer};
use utils::Random;
use network::GameAgent;
use ui::{Layout, Activation, Widget, WidgetData, ButtonState};

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
    
    let mut ui = Widget::new(WidgetData::Frame { outline: 0.0 })
        .with_primary(WHITE)
        .with_relative(
            Layout {
                center: vec2(400.0, 300.0),
                scale: vec2(800.0, 600.0)
            }
        )
        .with_child(
            Widget::new(WidgetData::Frame { outline: 0.0 })
                .with_relative(
                    Layout {
                        center: vec2(0., 0.),
                        scale: vec2(0.5, 0.8)
                    }
                )
                .with_child(
                    Widget::new(WidgetData::Button { state: ButtonState::Rest })
                        .with_id("join")
                        .with_primary(WHITE)
                        .with_relative(
                            Layout {
                                center: vec2(0., -0.26),
                                scale: vec2(0.4, 0.2)
                            }
                        )
                )
                .with_child(
                    Widget::new(WidgetData::Button { state: ButtonState::Rest })
                        .with_id("host")
                        .with_primary(WHITE)
                        .with_relative(
                            Layout {
                                center: vec2(0., -0.01),
                                scale: vec2(0.4, 0.2)
                            }
                        )
                )
                .with_child(
                    Widget::new(WidgetData::Button { state: ButtonState::Rest })
                        .with_id("quit")
                        .with_primary(WHITE)
                        .with_relative(
                            Layout {
                                center: vec2(0., 0.24),
                                scale: vec2(0.4, 0.2)
                            }
                        )
                )
        );

    ui.update_absolutes();
    
    'app: loop {
        let state;
        
        'menu: loop {
            
            clear_background(GREEN);
            
            for activation in ui.get_activations() {
                println!("{:?}", activation);
                
            }
            
            ui.draw();
            
            next_frame().await;
        }
                
        let mode = match state  {
            AppState::Finished => break 'app,
            AppState::Hosting  => Mode::Server,
            AppState::Joining  => Mode::Client
        };
        
        // default_game(mode).await;
        
    }
    
}
/*
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
                let mut s = s.borrow_mut().lock().unwrap();
                s.update();
                drop(s);
                
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

*/