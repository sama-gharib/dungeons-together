use std::collections::HashMap;

use desi_ui::*;
use uilang::uilang;
use macroquad::prelude::*;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum MenuVariant {
    Main,
    Join,
    Host,
    InGame,
    ConfirmQuit
}

pub struct Ui {
    data: HashMap<MenuVariant, Widget>,
    current: MenuVariant
}

impl Ui {
    const ACTIVATED_MENUS: [MenuVariant; 5] = [
        MenuVariant::Main,
        MenuVariant::Join,
        MenuVariant::Host,
        MenuVariant::InGame,
        MenuVariant::ConfirmQuit
    ];
    
    pub fn switch_menu(&mut self, next: MenuVariant) {
        if let Some(_) = self.data.get(&next) {
            self.current = next;
        } else {
            eprintln!("DEBUG: Tried to switch to an unexisting menu: {next:?}");
        }
    }
    
    pub fn tick(&mut self) {
        self.update();
        self.draw();
        self.check_activations();
    }
    
    fn update(&mut self) {
        self.data.get_mut(&self.current).unwrap().update_absolutes(
            Layout::new(
                vec2(screen_width()/2.0, screen_height()/2.0),
                vec2(screen_width(), screen_height())
            )
        );    
    }
    
    fn draw(&self) {
        self.data[&self.current].draw();
    }
    
    fn check_activations(&mut self) {
        let activations = self.data.get_mut(&self.current).unwrap().get_activations();
        
        activations
            .iter()
            .for_each( |x|
                self.switch_menu(self.current.apply(x.clone()))
            );
    }
}

impl Default for Ui {
    fn default() -> Self {
        
        let mut data = HashMap::new();
        Self::ACTIVATED_MENUS.iter().for_each(|x| { data.insert(*x, x.build_ui()); });
                        
        Self {
            data,
            current: Self::ACTIVATED_MENUS[0]
        }
    }
}

impl MenuVariant {
    pub fn build_ui(&self) -> Widget {
        match self {
            Self::Main => Self::main_menu(),
            MenuVariant::Main => Self::main_menu(),
            MenuVariant::Join => Self::join_menu(),
            MenuVariant::Host => Self::host_menu(),
            MenuVariant::InGame => Self::in_game_menu(),
            MenuVariant::ConfirmQuit => Self::confirm_quit_menu(),
        }
    }
    
    
    pub fn apply(&self, activation: Activation) -> Self {
        match self {
            Self::Main => match &activation.id[..]  {
                    "join" => Self::Join,
                    "host" => Self::Host,
                    "quit" => Self::ConfirmQuit,
                    _      => Self::Main
            },
            Self::Join => todo!(),
            Self::Host => todo!(),
            Self::InGame => todo!(),
            Self::ConfirmQuit => todo!(),
        }
    }
    
    fn main_menu() -> Widget {
        uilang!(
            <Frame>
                primary: "WHITE"
                <Frame>
                    scale: "(0.5, 0.8)"
                    <Button>
                        id: "join"
                        primary: "WHITE"
                        center: "(0.0, -0.26)"
                        scale: "(0.4, 0.2)"
                        <Label> text: "Join" </Label>
                    </Button>
                    <Button>
                        id: "host"
                        primary: "WHITE"
                        center: "(0.0, 0.0)"
                        scale: "(0.4, 0.2)"
                        <Label> text: "Host" </Label>
                    </Button>
                    <Button>
                        id: "quit"
                        primary: "WHITE"
                        center: "(0.0, 0.26)"
                        scale: "(0.4, 0.2)"
                        <Label> text: "Quit" </Label>
                    </Button>
                </Frame>
            </Frame>
        )
    }
    
    fn join_menu() -> Widget {
        uilang!(
            <Frame>
            </Frame>
        )
    }
    
    fn host_menu() -> Widget {
        uilang!(
            <Frame>
            </Frame>
        )
    }
    
    fn in_game_menu() -> Widget {
        uilang!(
            <Frame>
            </Frame>
        )
    }
    
    fn confirm_quit_menu() -> Widget {
        uilang!(
            <Frame>
            </Frame>
        )
    }
}