use macroquad::prelude::*;

use std::net::{ SocketAddr, TcpListener, TcpStream };
use std::io::{ Write, Error };
use std::thread::sleep;
use std::time::Duration;
use std::collections::VecDeque;

use crate::game::{ Controlable, Drawable, Dynamic };
use crate::utils::{ base_format, DefaultBehaviour, Random, Time };

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
    
    to_broadcast: VecDeque<(Command, usize)>,
    to_send: VecDeque<(Command, usize)>,
    
    connection_string: String
}

impl GameAgent for GameServer { }

impl Controlable for GameServer {
    fn handle_events(&mut self) {
        // TODO
    }
}

impl Drawable for GameServer {
    fn draw(&self) {
        draw_text("todo", 200.0, 200.0, 16.0, RED);
    }    
}

impl Dynamic for GameServer {
    fn update(&mut self) {
        print!("\rbroadcast queue length: {}                        ", self.to_broadcast.len());
        let _ = std::io::stdout().flush();
        
        self.accept_connection();
        self.receive_message();
        self.broadcast();
        self.send();
    }
}

impl GameServer {
    const SOCKET_DELAY: u64 = 16;
    
    
    pub fn new(connection_string: &str) -> Result<Self, Error> {
        let listener = TcpListener::bind(connection_string)?;
        listener.set_nonblocking(true)?;
        
        Ok(Self {
            listener,
            clients: Default::default(),
            to_broadcast: Default::default(),
            to_send: Default::default(),
            connection_string: connection_string.to_string()
        })
    }
    
    pub fn accept_connection(&mut self) {
        if let Ok(new_client) =  self.listener.accept() {
            if let Ok(()) = new_client.0.set_nonblocking(true) {
                //self.log(&format!("New client connected : {}", new_client.1));
                
                let new_id = Random::any();
                self.log(&format!("New client connected : {}", new_id));
                
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
                            disconnections.push(*id);
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
                        disconnections.push(index);
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
   
        
        for i in disconnections {
            self.log(&format!("Client {} disconnected.", self.clients[i].id));
            self.clients.remove(i);
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
                        println!("Failed to send a message : {e:?}");
                    }
                    break;
                }
            }
        }
    }
    
    fn log(&self, message: &str) {
        let (hour, minute, second) = Time::hour();
    
        println!(
            "[{}:{}:{}] {} > {message}",
            base_format(hour, 10),
            base_format(minute, 10),
            base_format(second, 10),
            self.connection_string
        )
    }
    
}