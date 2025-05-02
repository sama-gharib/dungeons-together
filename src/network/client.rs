use std::net::TcpStream;
use std::io::{ Write, Read };
use crate::utils::DefaultBehaviour;


pub struct GameClient {
    server: TcpStream
}

impl DefaultBehaviour for GameClient {
    fn default_behaviour(&mut self) {
    
    }
}

impl GameClient {
    pub fn new(connection_string: &str) -> Result<Self, std::io::Error> {
        let server = TcpStream::connect(connection_string)?;
        server.set_nonblocking(true)?;
        
        Ok(Self {
            server
        })
    }
    
    pub fn send(&mut self, msg: &str) {
        let _ = self.server.write_all(msg.as_bytes());
    }
    
    pub fn receive(&mut self) {
        let mut buffer = String::new();
        let _ = self.server.read_to_string(&mut buffer);
        println!("Received: {buffer}");
    }
}