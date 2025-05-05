use std::net::TcpStream;
use std::io::{ Write, Read };

use macroquad::prelude::*;

use crate::utils::DefaultBehaviour;
use crate::game::Body;

use super::{ Protocol, Command };

pub struct GameClient {
    server: TcpStream,
    player: Body
}

impl DefaultBehaviour for GameClient {
    fn default_behaviour(&mut self) {
        let to_send = Command::Reposition(5, vec2(10.0, 10.0));
        let _ = Protocol::send(&mut self.server, to_send);
        let _ = Protocol::reception(&mut self.server);
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
    
}