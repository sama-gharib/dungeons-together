use macroquad::prelude::*;

use std::net::{ SocketAddr, TcpListener, TcpStream };
use std::io::Error;
use std::thread::sleep;
use std::time::Duration;

use crate::utils::{ base_format, DefaultBehaviour, Random, Time };

use super::{Protocol, ProtocolError, Command};

#[derive(Debug)]
struct Client {
    stream: TcpStream,
    id: usize,
    position: Vec2
}

pub struct GameServer {
    listener: TcpListener,
    clients: Vec<Client>,
    
    to_broadcast: Vec<(Command, usize)>,
    to_send: Vec<(Command, usize)>,
    
    connection_string: String
}

impl DefaultBehaviour for GameServer {
    fn default_behaviour(&mut self) {
        self.accept_connection();
        self.receive_message();
        self.broadcast();
        self.send();
    }
}

impl GameServer {
    const SOCKET_DELAY: u64 = 4;
    
    
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
                    self.to_send.push((Command::Spawn(client.id), new_id));
                }
                  
                self.to_broadcast.push((Command::Spawn(new_id), new_id));
                
                self.clients.push(Client { stream: new_client.0,  id: new_id, position: Vec2::ZERO});
            } else {
                self.log("Failed to set client non-blocking.");
            }
        }
    }
    
    pub fn receive_message(&mut self) {
        
        let mut disconnections = Vec::new();
        
        for (index, client) in self.clients.iter_mut().enumerate() {
            
            match &mut Protocol::reception(&mut client.stream) {
                Ok( command)  => {
                    let should_broadcast = match command {
                        Command::Reposition(id, position) => {
                            client.position = *position;
                            *id = client.id;
                            true
                        },
                        _ => true
                    };
                    if should_broadcast {
                        self.to_broadcast.push((*command, client.id));
                    }
                },
                Err(e) => match e {
                    ProtocolError::Disconnection => {
                        disconnections.push(index);
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
        for (message, source) in self.to_broadcast.iter() {
            for target in self.clients.iter_mut() {
                if *source != target.id {
                    if let Ok(_) = Protocol::send(&mut target.stream, *message) {
                        success += 1;
                    } else {
                        error += 1;
                    }
                }
                sleep(Duration::from_millis(Self::SOCKET_DELAY));
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
        
        self.to_broadcast.clear();
    }

    pub fn send(&mut self) {
        for to_send in self.to_send.iter() {
            for client in self.clients.iter_mut() {
                if to_send.1 == client.id {
                    if let Err(e) = Protocol::send(&mut client.stream, to_send.0) {
                        println!("Failed to send a message : {e:?}");
                    }
                }
                sleep(Duration::from_millis(Self::SOCKET_DELAY));
            }
        }
        
        self.to_send.clear();
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