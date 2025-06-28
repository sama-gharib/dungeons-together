use macroquad::prelude::*;

use std::net::{ TcpListener, TcpStream };
use std::io::{ Write, Error };
use std::collections::VecDeque;

use crate::game::component::GameComponent;
use crate::game::map::Map;
use crate::game::object::GameObject;
use crate::utils::{ Controlable, Drawable, Dynamic };
use crate::utils::{ base_format, Random, Time };

use super::{Command, GameAgent, Protocol, ProtocolError};

#[derive(Debug)]
struct Client {
    stream: TcpStream,
    id: usize,
    position: Vec2,
    protocol: Protocol
}

pub struct GameServer {
    listener: TcpListener,
    clients: Vec<Client>,
    
    map_seed: usize,
    map: Map,
    monster: GameComponent,
    
    to_broadcast: VecDeque<(Command, usize)>,
    to_send: VecDeque<(Command, usize)>,
    
    connection_string: String
}

impl GameAgent for GameServer { }

impl Controlable for GameServer {
    fn handle_events(&mut self) -> bool {
        self.monster.slide(&self.map);
        
        false
    }
}

impl Drawable for GameServer {
    fn draw(&self) {
        draw_text("todo", 200.0, 200.0, 48.0, RED);
        
        for client in self.clients.iter() {
            draw_rectangle(client.position.x, client.position.y, 100.0, 100.0, BLUE);
        }
    }    
}

impl Dynamic for GameServer {
    fn update(&mut self) {
        print!("\rbroadcast queue length: {}                        ", self.to_broadcast.len());
        let _ = std::io::stdout().flush();
        
        if Random::max(30) == 0 {
            self.to_broadcast.push_back((Command::Reposition(0, self.monster.body.position), 0));
        }
        
        self.accept_connection();
        self.receive_message();
        self.broadcast();
        self.send();
    }
}

impl GameServer {
    
    pub const MAP_WIDTH: usize = 50;
    pub const MAP_HEIGHT: usize = 50;
    
    pub fn new(connection_string: &str) -> Result<Self, Error> {
        let listener = TcpListener::bind(connection_string)?;
        listener.set_nonblocking(true)?;
        
        let seed = Random::any();
        
        Ok(Self {
            listener,
            clients: Default::default(),
            map_seed: seed,
            map: Map::generate(Self::MAP_WIDTH, Self::MAP_HEIGHT, seed),
            monster: GameComponent::from(GameObject::Monster),
            to_broadcast: Default::default(),
            to_send: Default::default(),
            connection_string: connection_string.to_string()
        })
    }
    
    pub fn accept_connection(&mut self) {
        if let Ok(new_client) =  self.listener.accept() {
            if let Ok(()) = new_client.0.set_nonblocking(true) {
                
                let new_id = Random::any();
                self.log(&format!("New client connected : {}", new_id));
                self.to_send.push_back((Command::Spawn(0), new_id));
                self.to_send.push_back((Command::ChangeMap(self.map_seed), new_id));
                for client in self.clients.iter() {
                    self.to_send.push_back((Command::Spawn(client.id), new_id));
                }
                  
                self.to_broadcast.push_back((Command::Spawn(new_id), new_id));
                
                self.clients.push(Client { stream: new_client.0,  id: new_id, position: Vec2::ZERO, protocol: Protocol::new()});
                
            } else {
                self.log("Failed to set client non-blocking.");
            }
        }
    }
    
    pub fn receive_message(&mut self) {
        
        let mut disconnections = Vec::new();
        
        for (index, client) in self.clients.iter_mut().enumerate() {
            
            match &mut client.protocol.reception(&mut client.stream) {
                Ok(command)  => {
                    let should_broadcast = match command {
                        Command::Reposition(id, position) => {
                            client.position = *position;
                            *id = client.id;
                            true
                        },
                        Command::Despawn(id) => {
                            *id = client.id;
                            disconnections.push((index, *id));
                            true  
                        },
                        _ => true
                    };
                    if should_broadcast {
                        self.to_broadcast.push_back((*command, client.id));
                    }
                },
                Err(e) => match e {
                    ProtocolError::Disconnection => {
                        self.to_broadcast.push_back((Command::Despawn(client.id), client.id));
                        disconnections.push((index, client.id));
                    },
                    ProtocolError::OutdatedPackage => {
                        // TODO
                    },
                    ProtocolError::IllFormatedSequenceNumber => {
                        // TODO
                    },
                    ProtocolError::WrongSequence => {
                        // TODO
                    }
                }
            }
        }
   
        
        for (index, id) in disconnections {
            self.log(&format!("Client {} disconnected.", id));
            self.clients.remove(index);
        }
    }
    
    pub fn broadcast(&mut self) {
        
        let mut success = 0;
        let mut error = 0;
        if let Some((message, source)) = self.to_broadcast.pop_front() {
            for target in self.clients.iter_mut() {
                if source != target.id {
                    if let Ok(_) = target.protocol.send(&mut target.stream, message) {
                        success += 1;
                    } else {
                        error += 1;
                    }
                }
            }
        }
        
        if error > 0 {
            self.log(
                    &format!
                    (
                        "An error occured while broadcasting {} messages to {} clients.\n\tSuccess: {success}\n\tFailures: {error}",
                        success + error,
                        self.clients.len()
                    )
            );
        }
        
    }

    pub fn send(&mut self) {
        if let Some((command, target)) = self.to_send.pop_front() {
            for client in self.clients.iter_mut() {
                if target == client.id {
                    if let Err(e) = client.protocol.send(&mut client.stream, command) {
                        println!("\rFailed to send a message : {e:?}                               ");
                    }
                    break;
                }
            }
        }
    }
    
    fn log(&self, message: &str) {
        let (hour, minute, second) = Time::hour();
    
        println!(
            "\r[{}:{}:{}] {} > {message}                                                ",
            base_format(hour, 10),
            base_format(minute, 10),
            base_format(second, 10),
            self.connection_string
        )
    }
    
}