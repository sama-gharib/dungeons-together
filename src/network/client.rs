use std::net::TcpStream;
use std::collections::HashMap;

use macroquad::prelude::*;

use crate::utils::{ DefaultBehaviour };
use crate::game::{ Body, Drawable };

use super::{ Protocol, Command };

pub struct GameClient {
    server: TcpStream,
    player: Rect,
    others: HashMap<usize, Rect>
}

impl DefaultBehaviour for GameClient {
    fn default_behaviour(&mut self) {
        let to_send = Command::Reposition(0, self.player.point());
        let _ = Protocol::send(&mut self.server, to_send);
        
        self.receive();
    
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
        
        self.draw();
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

impl GameClient {
    pub fn new(connection_string: &str) -> Result<Self, std::io::Error> {
        let server = TcpStream::connect(connection_string)?;
        server.set_nonblocking(true)?;
        
                
        Ok(Self {
            server,
            player: Rect { x: 100.0, y: 100.0, w: 100.0, h: 100.0 },
            others: HashMap::new()
        })
    }
 
    fn receive(&mut self) {
        if let Ok(command) = Protocol::reception(&mut self.server) {
            match command {
                Command::Spawn(id) => {
                    self.others.insert(id, Rect { x: 100.0, y: 100.0, w: 100.0, h: 100.0 });
                },
                Command::Reposition(id, pos) => if let Some(other) = self.others.get_mut(&id) {
                    other.x = pos.x;
                    other.y = pos.y;
                },
                Command::EndGame => todo!(),
                Command::Unknown => todo!(),
                Command::IllFormated(_) => todo!()
            }
        }
    }
}