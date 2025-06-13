use std::mem;

use macroquad::window::next_frame;
use menu::{MenuVariant, Ui};

use crate::network::{
    GameAgent,
    client::GameClient,
    server::GameServer
};

mod menu;

#[derive(Default)]
pub struct Application {
    ui: Ui,
    game: Option<Box<dyn GameAgent>>
}


impl Application {
    pub async fn run(&mut self) {
        loop {
            
            let last = self.ui.get_current();
            self.ui.tick();
            let current = self.ui.get_current_mut();
            
            if let Some(game) = &mut self.game {
                game.handle_events();
                game.update();
                game.draw();
            }
            
            // UI transitions special behaviours
            match (last, current.clone()) {
                (MenuVariant::Join { name, ip, port }, MenuVariant::InGame ) => {
                    let (_name, ip, port) = (name.unwrap(), ip.unwrap(), port.unwrap());
                    
                    let client = GameClient::new(&format!("{ip}:{port}"));
                    
                    match client {
                        Ok(client) => {
                            self.game = Some(Box::new(client));
                        },
                        Err(e) => {
                            eprintln!("DEBUG: Failed to connect : {e:?}");
                            *current = MenuVariant::Join { name: None, ip: None, port: None }
                        }
                }
                },
                (MenuVariant::Host { port }, MenuVariant::InGame) => {
                    self.game = Some(Box::new(GameServer::new(&format!("0.0.0.0:{}", port.unwrap())).unwrap()));
                },
                (MenuVariant::InGame, MenuVariant::InGame) => {},
                (MenuVariant::InGame, _) => { self.game = None },
                _ => {}
            }
            
            if self.ui.is_terminated() {
                break;
            }
            
            next_frame().await;
        }
    }    
}