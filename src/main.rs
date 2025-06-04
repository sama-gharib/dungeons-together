use application::Application;

use utils::Random;

mod utils;
mod network;
mod game;
mod application;


#[macroquad::main("Bored")]
async fn main() {    
    Random::seed();
    let mut app = Application::default();
    app.run().await;
    println!("Finished !");
}

/*
async fn default_game(mode: Mode) {
    
    let mut s: Arc<Mutex<dyn GameAgent + Send>> = match mode {
        Mode::Server => Arc::new(Mutex::new(GameServer::new("0.0.0.0:53000").unwrap())),
        Mode::Client => Arc::new(Mutex::new(GameClient::new("localhost:53000").unwrap()))
    };
    
    let mut should_stop = Arc::new(Mutex::new(false));
    
    let backend_thread = thread::spawn( {
        let mut should_stop: Arc<Mutex<bool>> = Arc::clone(&should_stop);
        let mut s = Arc::clone(&s);
        move || {
            loop {
                let mut s = s.borrow_mut().lock().unwrap();
                s.update();
                drop(s);
                
                sleep(Duration::from_millis(8));
                if *should_stop.borrow_mut().lock().unwrap() {
                    break;
                }
            }
        }
    });
    
    let mut ui = uilang!(
        <Button>
            id: "menu"
            center: "(0.4, -0.4)"
            scale: "(0.15, 0.1)"
            primary: "WHITE"
            secondary: "BLUE"
            <Label> text: "Menu" primary: "BLACK" </Label>
        </Button>
    );
    
    ui.update_absolutes(Layout::new(vec2(400.0, 300.0), vec2(800.0, 600.0)));
    
    'game: loop {            
        clear_background(BLACK);
        draw_text("Work in progress.", 100., 100., 32., WHITE);
        
        if let Ok(mut s) = s.borrow_mut().lock() {
            s.handle_events();
            s.draw();
        }
        
        ui.draw();
        for activation in ui.get_activations() {
            println!("{activation:?}");
            if activation.id == "menu" {
                break 'game;
            }
        }
        
        if *should_stop.borrow_mut().lock().unwrap() {
            break;
        }
        
        next_frame().await;
    }
    
    *(should_stop.borrow_mut().lock().unwrap()) = true;
    
    if let Err(e) = backend_thread.join() {
        println!("Failed to join backend thread : {e:?}");
    }
    
}
*/