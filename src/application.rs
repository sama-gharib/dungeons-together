use macroquad::window::next_frame;
use menu::{MenuVariant, Ui};

use crate::network::{
    GameAgent,
    client::GameClient
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
                (MenuVariant::Host, MenuVariant::InGame) => todo!(),
                (_, _) => {}
            }
            
            if self.ui.is_terminated() {
                break;
            }
            
            next_frame().await;
        }
    }    
}