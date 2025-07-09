use macroquad::prelude::*;

use std::collections::VecDeque;
use std::net::{ TcpListener, TcpStream };
use std::io::Error;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::utils::{ Controlable, Drawable, Dynamic };
use crate::utils::{ base_format, Random, Time };

use super::{Command, GameAgent, Protocol, ProtocolError};

struct Message {
    body: Command,
    source: usize,
    read_by: Vec<usize>
}

struct Client {
    stream: TcpStream,
    
    id: usize,
    protocol: Protocol,
    
    disconnected: bool
}

pub struct GameServer {    
    map_seed: usize,
    
    clients: Vec<(JoinHandle<()>, usize)>,
    listener: TcpListener,
    
    broadcast_queue: Arc<Mutex<VecDeque<Message>>>,
}

impl GameAgent for GameServer {}
impl Controlable for GameServer {
    fn handle_events(&mut self) -> bool {
        // TODO: Command prompt maybe ?
        false
    }
}

impl Drawable for GameServer {
    fn draw(&self) {
        // TODO
    }
}

impl Dynamic for GameServer {
    fn update(&mut self) {
        self.accept_connections();
        
        let mut queue = self.broadcast_queue.lock().unwrap();
        
        // Removing messages read by every clients
        if !queue.is_empty() {
            for i in queue.len()-1..=0 {
                if queue[i].read_by.len() >= self.clients.len() {
                    queue.remove(i);
                }
            }
        }
    }
}

impl Client {
    
    fn tick(&mut self, message_queue: &mut VecDeque<Message>) {
        self.receive(message_queue);
        self.send(message_queue);
    }
    
    fn receive(&mut self, message_queue: &mut VecDeque<Message>) {
        match &mut self.protocol.reception(&mut self.stream) {
            Ok(command) => {
                match command {
                    Command::Spawn(id) | Command::Reposition(id, _) => { *id = self.id; },
                    _ => {}
                }
                message_queue.push_back(Message { body: *command, source: self.id, read_by: vec![self.id] })
            },
            Err(e) => match e {
                ProtocolError::Disconnection => self.disconnected = true,
                ProtocolError::WrongSequence => {
                    // TODO
                },
                ProtocolError::OutdatedPackage => {
                    // TODO
                },
                ProtocolError::IllFormatedSequenceNumber => {
                    // TODO
                },
            }
        }
    }
    
    fn send(&mut self, message_queue: &mut VecDeque<Message>) {
        for message in message_queue.iter_mut() {
            if message.source != self.id && !message.read_by.contains(&self.id) {
                message.read_by.push(self.id);
                match self.protocol.send(&mut self.stream, message.body) {
                    Ok(_) => {},
                    Err(e) => GameServer::log(&format!("Error while sending message: {e:?}")),
                }
            }
        }
    }
}

impl GameServer {
    
    pub const MAP_WIDTH: usize = 50;
    pub const MAP_HEIGHT: usize = 50;
    
    pub fn new(connection_string: &str) -> Result<Self, Error> {
        let listener = TcpListener::bind(connection_string)?;
        listener.set_nonblocking(true)?;
        
        Ok(Self {
            map_seed: Random::any(),
            clients: Vec::default(),
            listener,
            broadcast_queue: Arc::new(Mutex::new(VecDeque::default())),
        })
    }
    
    pub fn accept_connections(&mut self) {
        if let Ok(mut stream) = self.listener.accept() {
            stream.0.set_nonblocking(true).unwrap();
            
            // Sending initial messages (map seed and other players)
            let mut new_protocol = Protocol::new();
            new_protocol.send(&mut stream.0, Command::ChangeMap(self.map_seed)).unwrap();
            for (_, id) in self.clients.iter() {
                new_protocol.send(&mut stream.0, Command::Spawn(*id)).unwrap();
            }

            self.clients.push({
                let new_id = Random::any();
                let client = Client {
                    stream: stream.0,
                    id: new_id,
                    protocol: new_protocol,
                    disconnected: false
                };
                let queue = Arc::clone(&self.broadcast_queue);
                
                {
                    // Broadcasting spawn command to other players
                    queue
                        .lock()
                        .unwrap()
                        .push_back(Message {
                            body: Command::Spawn(new_id),
                            source: new_id,
                            read_by: Vec::default()
                        });
                }
                
                (thread::spawn(move || Self::tick_client(client, queue)), new_id)
            });
        }
    }
    
    fn tick_client(mut client: Client, broadcast_queue: Arc<Mutex<VecDeque<Message>>>) {
        loop {
            thread::sleep(Duration::from_millis(1));
            
            let mut queue = broadcast_queue.lock().unwrap();
            client.tick(&mut queue);
            if client.disconnected {
                Self::log(&format!("Client {} disconnected", client.id));
                queue.push_back(Message {
                    body: Command::Despawn(client.id),
                    source: client.id,
                    read_by: Vec::default()
                });
                break;
            }
        }
    }
    
    fn log(message: &str) {
        let (hour, minute, second) = Time::hour();
    
        println!(
            "\r[{}:{}:{}] > {message}                                                ",
            base_format(hour, 10),
            base_format(minute, 10),
            base_format(second, 10)
        )
    }
}
