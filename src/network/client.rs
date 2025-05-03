use std::net::TcpStream;
use std::io::{ Write, Read };

use macroquad::prelude::*;

use crate::utils::DefaultBehaviour;
use crate::game::Body;

pub struct GameClient {
    server: TcpStream,
    player: Body
}

impl DefaultBehaviour for GameClient {
    fn default_behaviour(&mut self) {
        self.send(&format!("\u{01}{};{}\0", self.player.position.x, self.player.position.y));
        self.receive();
    }
}

impl GameClient {
    pub fn new(connection_string: &str) -> Result<Self, std::io::Error> {
        let server = TcpStream::connect(connection_string)?;
        server.set_nonblocking(true)?;
        
        Ok(Self {
            server,
            player: Body::new(vec2(100., 100.), vec2(100., 100.))
        })
    }
    
    pub fn send(&mut self, msg: &str) {
        let _ = self.server.write_all(msg.as_bytes());
    }
    
    pub fn receive(&mut self) {
        todo!(); // THIS SHOULD LOOK LIKE GameServer::receive_message
        let mut buffer = String::new();
        if let Ok(bytes_read) = self.server.read_to_string(&mut buffer) {
            if bytes_read == 0 {
                println!("Server disconnected.");
            } else {
                println!("Received: {buffer}");
            }
        }
    }
}