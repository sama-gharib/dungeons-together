use std::borrow::BorrowMut;
use std::net::TcpStream;
use std::collections::HashMap;
use std::sync::{ Mutex, Arc };
use std::time::Duration;
use std::borrow::Borrow;

use macroquad::prelude::*;

use crate::game::{ Body, Controlable, Drawable, Dynamic };

use super::{ Protocol, Command, GameAgent };

use std::thread::{self, JoinHandle};

pub struct GameClient {
    network_thread: JoinHandle<()>,
    player: Rect,
    others: HashMap<usize, Rect>,
    has_moved: bool,
    
    // Thread safe data
    running: Arc<Mutex<bool>>,
    inbox: Arc<Mutex<Vec<Command>>>,
    to_send: Arc<Mutex<Vec<Command>>>
}

impl Controlable for GameClient {
    fn handle_events(&mut self) {
        let last_pos = self.player.point();
        if is_key_down(KeyCode::Right) {
            self.player.x += 5.0;
        }
        if is_key_down(KeyCode::Left) {
            self.player.x -= 5.0;
        }
        if is_key_down(KeyCode::Up) {
            self.player.y -= 5.0;
        }
        if is_key_down(KeyCode::Down) {
            self.player.y += 5.0;
        }
        if self.player.point() != last_pos {
            self.has_moved = true;
        }
    }
}

impl Drawable for GameClient {
    fn draw(&self) {
        for (id, r) in self.others.iter() {
            draw_rectangle(
                r.x,
                r.y,
                r.w,
                r.h,
                BLUE);
            draw_text(&id.to_string(), r.x + 10.0, r.y + 10.0, 13.0, WHITE);
        }
        
        draw_rectangle(self.player.x, self.player.y, self.player.w, self.player.h, WHITE);
    }
}

impl GameAgent for GameClient {}

impl Dynamic for GameClient {
    fn update(&mut self) {
        if self.has_moved {
            if let Ok(mut to_send) = self.to_send.borrow_mut().lock() {
                to_send.push(Command::Reposition(0, self.player.point()));
                self.has_moved = false;
            }
        }
        
        self.receive();
    }
}

impl Drop for GameClient {
    fn drop(&mut self) {
        let _ = self.to_send.borrow_mut().lock().unwrap().push(Command::Despawn(0));
        (*self.running.borrow_mut().lock().unwrap()) = false;
        let _ = std::mem::replace(&mut self.network_thread, std::thread::spawn(|| {1;})).join();
    }
}

impl GameClient {
    pub fn new(connection_string: &str) -> Result<Self, std::io::Error> {

        let inbox = Arc::new(Mutex::new(Vec::new()));
        let to_send = Arc::new(Mutex::new(Vec::new()));
        let running = Arc::new(Mutex::new(true));
        
        let network_thread = {
            let connection_string = connection_string.to_string();
            let inbox = inbox.clone();
            let to_send = to_send.clone();
            let running = running.clone();
            
            std::thread::spawn(move || Self::network_worker(connection_string, inbox, to_send, running))
        };
        
        Ok(Self {
            network_thread,
            player: Rect { x: 100.0, y: 100.0, w: 100.0, h: 100.0 },
            others: HashMap::new(),
            has_moved: true,
            running,
            to_send,
            inbox
        })
    }
    
    fn network_worker(
        connection_string: String,
        mut inbox: Arc<Mutex<Vec<Command>>>,
        mut to_send: Arc<Mutex<Vec<Command>>>,
        mut running: Arc<Mutex<bool>>
    ) {
        let mut server = TcpStream::connect(connection_string).unwrap();
        server.set_nonblocking(true).unwrap();
        
        let mut protocol = Protocol::new();
        
        loop {
            // Reception
            if let Ok(command) = protocol.reception(&mut server) {
                inbox.borrow_mut().lock().unwrap().push(command);
            }
            
            // Sending
            if let Some(to_send) = to_send.borrow_mut().lock().unwrap().pop() {
                let _ = protocol.send(&mut server, to_send);
            }
            
            // End of thread condition
            if *running.borrow_mut().lock().unwrap() == false {
                break;
            }
            
            std::thread::sleep(Duration::from_millis(16));
        }
    }
 
    fn receive(&mut self) {
        if let Some(command) = self.inbox.borrow_mut().lock().unwrap().pop() {
            match command {
                Command::Spawn(id) => {
                    self.others.insert(id, Rect { x: 100.0, y: 100.0, w: 100.0, h: 100.0 });
                },
                Command::Reposition(id, pos) => if let Some(other) = self.others.get_mut(&id) {
                    other.x = pos.x;
                    other.y = pos.y;
                },
                Command::Despawn(id) => {
                    self.others.remove(&id);
                },
                Command::Unknown => todo!(),
                Command::IllFormated(_) => todo!()
            }
        }
    }
}