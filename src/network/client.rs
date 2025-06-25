use std::borrow::BorrowMut;
use std::net::{ SocketAddr, TcpStream, ToSocketAddrs};
use std::collections::HashMap;
use std::sync::{ Mutex, Arc };
use std::time::Duration;
use std::io::ErrorKind;
use std::thread::JoinHandle;

use macroquad::prelude::*;

use crate::game::{
    component::*,
    subject::GameSubject,
    map::Map
};
use crate::utils::{ Controlable, Drawable, Dynamic };

use super::server::GameServer;
use super::{ Protocol, Command, GameAgent };


pub struct GameClient {
    network_thread: JoinHandle<()>,
    
    player: GameComponent,
    others: HashMap<usize, Rect>,
    map: Map,
    camera: Camera2D,
    
    // Thread safe data
    running: Arc<Mutex<bool>>,
    inbox: Arc<Mutex<Vec<Command>>>,
    to_send: Arc<Mutex<Vec<Command>>>
}

#[derive(Debug, Clone, Copy)]
pub enum ClientConnectionError {
    UnableToResolve,
    ServerNotFound,
    ElapsedTimeout,
    ServerRefused
}

impl Controlable for GameClient {
    fn handle_events(&mut self) -> bool {
        self.player.handle_events()
    }
}

impl Drawable for GameClient {
    fn draw(&self) {
        set_camera(&self.camera);
        
        for (id, r) in self.others.iter() {
            draw_rectangle(
                r.x,
                r.y,
                r.w,
                r.h,
                BLUE);
            draw_text(&id.to_string(), r.x + 10.0, r.y + 10.0, 13.0, YELLOW);
        }
        
        self.player.draw();
        
        for room in self.map.rooms.iter() {
            for wall in room.components
                .iter()
                .filter(|x| x.is_some())
                .map(|x| x.as_ref().unwrap()) {
                wall.draw()
            }
        }
        
        set_default_camera();
    }
}

impl GameAgent for GameClient {}

impl Dynamic for GameClient {
    fn update(&mut self) {
      
        let last_pos = self.player.body().position;
        self.player.update();
        let current_pos = self.player.body().position;
        
        self.camera.target = Vec2::lerp(self.camera.target, current_pos, 0.1);
        
        if current_pos != last_pos {
            if let Ok(mut to_send) = self.to_send.borrow_mut().lock() {
                to_send.push(Command::Reposition(0, self.player.body().position()));
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
    pub fn new(connection_string: &str) -> Result<Self, ClientConnectionError> {
        
        let inbox = Arc::new(Mutex::new(Vec::new()));
        let to_send = Arc::new(Mutex::new(Vec::new()));
        let running = Arc::new(Mutex::new(true));
        
        let network_thread = {
            let connection_string = connection_string.to_string();
            let inbox = inbox.clone();
            let to_send = to_send.clone();
            let running = running.clone();
            
            // Performing DNS lookup on connection_string
            let address: SocketAddr = match connection_string.to_socket_addrs() {
                Ok(mut addrs) => match addrs.next() {
                    Some(addr) => addr,
                    None => return Err(ClientConnectionError::ServerNotFound)   
                },
                Err(_) => return Err(ClientConnectionError::UnableToResolve)
            };
            
            // Connecting to server
            let server = match TcpStream::connect_timeout(
                &address,
                Duration::from_secs(2)
            ) {
                Ok(server) => server,
                Err(e) => match e.kind() {
                    ErrorKind::TimedOut => return Err(ClientConnectionError::ElapsedTimeout),
                    ErrorKind::ConnectionRefused => return Err(ClientConnectionError::ServerRefused),
                    _ => panic!("Unahandled client connection error !")
                }
            };
            
            std::thread::spawn(move || Self::network_worker(server, inbox, to_send, running))
        };
        
        Ok(Self {
            network_thread,
            player: GameComponent::from(
                GameComponentVariant::Subject(
                    GameSubject::default()
                )
            ),
            others: HashMap::new(),
            map: Default::default(),
            camera: Camera2D::from_display_rect(Rect { x: 0.0, y: 600.0, w: 800.0, h: -600.0 }),
            running,
            to_send,
            inbox
        })
    }
    
    fn network_worker(
        mut server: TcpStream,
        mut inbox: Arc<Mutex<Vec<Command>>>,
        mut to_send: Arc<Mutex<Vec<Command>>>,
        mut running: Arc<Mutex<bool>>
    ) {
        
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
                    let before = other.point();
                    let after = before + (pos - before) / 4.0;
                    other.x = after.x;
                    other.y = after.y;
                },
                Command::Despawn(id) => {
                    self.others.remove(&id);
                },
                Command::ChangeMap(seed) => {
                    self.map = Map::generate(GameServer::MAP_WIDTH, GameServer::MAP_HEIGHT, seed);
                },
                Command::Unknown => todo!(),
                Command::IllFormated(_) => todo!()
            }
        }
    }
}