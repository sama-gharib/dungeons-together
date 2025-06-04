use macroquad::window::next_frame;
use menu::Ui;

mod menu;

#[derive(Default)]
pub struct Application {
    ui: Ui
}


impl Application {
    pub async fn run(&mut self) {
        loop {
            
            self.ui.tick();
            
            
            if self.ui.is_terminated() {
                break;
            }
            
            next_frame().await;
        }
    }    
}