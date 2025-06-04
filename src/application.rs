use macroquad::window::next_frame;
use menu::{MenuVariant, Ui};

mod menu;

#[derive(Default)]
pub struct Application {
    ui: Ui
}


impl Application {
    pub async fn run(&mut self) {
        loop {
            
            let last = self.ui.get_current();
            self.ui.tick();
            let current = self.ui.get_current();
            
            // Transition special behaviours
            match (last, current) {
                (MenuVariant::Join, MenuVariant::InGame) => todo!(),
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