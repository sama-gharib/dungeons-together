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
    ConfirmQuit,
    Oblivion
}

pub struct Ui {
    data: HashMap<MenuVariant, Widget>,
    current: MenuVariant,
    terminated: bool
}

impl Ui {
    const ACTIVATED_MENUS: [MenuVariant; 6] = [
        MenuVariant::Main,
        MenuVariant::Join,
        MenuVariant::Host,
        MenuVariant::InGame,
        MenuVariant::ConfirmQuit,
        MenuVariant::Oblivion
    ];
    
    pub fn is_terminated(&self) -> bool {
        self.terminated
    }
    
    pub fn switch_menu(&mut self, next: MenuVariant) {
        if let Some(_) = self.data.get(&next) {
            self.current = next;
        } else {
            eprintln!("DEBUG: Tried to switch to an unexisting menu: {next:?}");
        }
    }
    
    pub fn tick(&mut self) {
        
        if let MenuVariant::Oblivion = self.current {
            self.terminated = true;
        } else {
            self.update();
            self.draw();
            self.check_activations();
        }
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
            current: Self::ACTIVATED_MENUS[0],
            terminated: false
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
            MenuVariant::Oblivion => Widget::default()
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
            Self::Join => match &activation.id[..] {
                "back" => Self::Main,
                _ => todo!()
            },
            Self::Host => match &activation.id[..] {
                "back" => Self::Main,
                _ => todo!()
            },
            Self::InGame => match &activation.id[..] {
                "back" => Self::Main,
                _ => todo!()
            },
            Self::ConfirmQuit => match &activation.id[..] {
                "back" => Self::Main,
                _ => Self::Oblivion
            },
            Self::Oblivion => panic!("Ui was ticked after having requested termination")
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
                primary: "WHITE"
                <Label> text: "Not yet implemented" </Label>
                <Button>
                    id: "back"
                    center: "(0.4, -0.4)"
                    scale: "(0.1, 0.1)"
                    primary: "GRAY"
                    secondary: "BLACK"
                    <Label>
                        text: "Main menu"
                    </Label>
                </Button>
            </Frame>
        )
    }
    
    fn host_menu() -> Widget {
        uilang!(
            <Frame>
                primary: "WHITE"
                <Label> text: "Not yet implemented" </Label>
                <Button>
                    id: "back"
                    center: "(0.4, -0.4)"
                    scale: "(0.1, 0.1)"
                    primary: "GRAY"
                    secondary: "BLACK"
                    <Label>
                        text: "Main menu"
                    </Label>
                </Button>
            </Frame>
        )
    }
    
    fn in_game_menu() -> Widget {
        uilang!(
            <Frame>
                primary: "WHITE"
                <Label> text: "Not yet implemented" </Label>
                <Button>
                    id: "back"
                    center: "(0.4, -0.4)"
                    scale: "(0.1, 0.1)"
                    primary: "GRAY"
                    secondary: "BLACK"
                    <Label>
                        text: "Main menu"
                    </Label>
                </Button>
            </Frame>
        )
    }
    
    fn confirm_quit_menu() -> Widget {
        uilang!(
            <Frame>
                primary: "WHITE"
                <Label>
                    text: "Quit ?"
                    center: "(0.0, -0.3)"
                </Label>
                <Frame>
                    center: "(0.0, 0.1)"
                    scale: "(0.4, 0.3)"
                    <Button>
                        id: "back"
                        center: "(-0.25, 0.0)"
                        scale: "(0.5, 1.0)"
                        primary: "GREEN"
                        <Label>
                            text: "No"
                        </Label>
                    </Button>
                    <Button>
                        id: "quit"
                        center: "(0.25, 0.0)"
                        scale: "(0.5, 1.0)"
                        primary: "RED"
                        <Label>
                            text: "Yes"
                        </Label>
                    </Button>
                </Frame>
            </Frame>
        )
    }
}