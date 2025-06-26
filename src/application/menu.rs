use desi_ui::*;
use uilang::uilang;
use macroquad::prelude::*;

use crate::utils::DiscriminantMap;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum MenuVariant {
    Main,
    Join { name: Option<String>, ip: Option<String>, port: Option<String> },
    Host { port: Option<String> },
    InGame,
    ConfirmQuit,
    Oblivion
}

pub struct Ui {
    data: DiscriminantMap<MenuVariant, Widget>,
    current: MenuVariant,
    terminated: bool
}

impl Ui {
    const ACTIVATED_MENUS: [MenuVariant; 6] = [
        MenuVariant::Main,
        MenuVariant::Join { name: None, ip: None, port: None },
        MenuVariant::Host { port: None },
        MenuVariant::InGame,
        MenuVariant::ConfirmQuit,
        MenuVariant::Oblivion
    ];
    
    pub fn is_terminated(&self) -> bool {
        self.terminated
    }
    pub fn get_current(&self) -> MenuVariant {
        self.current.clone()
    }
    pub fn get_current_mut(&mut self) -> &mut MenuVariant {
        &mut self.current
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
        
        let mut data = DiscriminantMap::default();
        Self::ACTIVATED_MENUS.iter().for_each(|x| { data.push(x.clone(), x.build_ui()); });
                        
        Self {
            data,
            current: Self::ACTIVATED_MENUS[0].clone(),
            terminated: false
        }
    }
}

impl MenuVariant {
    pub fn build_ui(&self) -> Widget {
        match self {
            MenuVariant::Main => Self::main_menu(),
            MenuVariant::Join { .. } => Self::join_menu(),
            MenuVariant::Host { .. } => Self::host_menu(),
            MenuVariant::InGame { .. } => Self::in_game_menu(),
            MenuVariant::ConfirmQuit => Self::confirm_quit_menu(),
            MenuVariant::Oblivion => Widget::default()
        }
    }
    
    
    pub fn apply(&self, activation: Activation) -> Self {
        match self {
            Self::Main => match &activation.id[..]  {
                "join" => Self::Join { name: None, ip: None, port: None } ,
                "host" => Self::Host { port: None },
                "quit" => Self::ConfirmQuit,
                _      => Self::Main
            },
            Self::Join { name, ip, port } => match &activation.id[..] {
                "back" => Self::Main,
                "name" => {
                    Self::Join {name: Some(activation.message.unwrap()), ip: ip.clone(), port: port.clone()}
                },
                "ip" => {
                    Self::Join {name: name.clone(), ip: Some(activation.message.unwrap()), port: port.clone()}
                },
                "port" => {
                    Self::Join {name: name.clone(), ip: ip.clone(), port: Some(activation.message.unwrap())}
                },
                "join" => {
                    Self::InGame
                },
                _ => { todo!() }
            },
            Self::Host { .. } => match &activation.id[..] {
                "back" => Self::Main,
                "start" => Self::InGame,
                "port" => Self::Host { port: Some(activation.message.unwrap()) },
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
                    primary: "WHITE"
                    secondary: "BLACK"
                    outline: "2.0"
                    scale: "(0.5, 0.8)"
                    <Button>
                        id: "join"
                        primary: "GRAY"
                        secondary: "DARKGRAY"
                        center: "(0.0, -0.26)"
                        scale: "(0.4, 0.2)"
                        <Label> text: "Join" </Label>
                    </Button>
                    <Button>
                        id: "host"
                        primary: "GRAY"
                        secondary: "DARKGRAY"
                        center: "(0.0, 0.0)"
                        scale: "(0.4, 0.2)"
                        <Label> text: "Host" </Label>
                    </Button>
                    <Button>
                        id: "quit"
                        primary: "GRAY"
                        secondary: "DARKGRAY"
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
                <Label>
                    text: "Join"
                    center: "(0.0, -0.4)"
                    scale: "(0.5, 0.2)"
                </Label>
                <TextInput>
                    id: "name"
                    placeholder: "name"
                    primary: "WHITE"
                    center: "(0.0, -0.25)"
                    scale: "(0.5, 0.1)"
                </TextInput>
                <TextInput>
                    id: "ip"
                    placeholder: "0.0.0.0"
                    primary: "WHITE"
                    center: "(0.0, -0.05)"
                    scale: "(0.5, 0.1)"
                </TextInput>
                <TextInput>
                    id: "port"
                    placeholder: "53000"
                    primary: "WHITE"
                    center: "(0.0, 0.15)"
                    scale: "(0.5, 0.1)"
                </TextInput>
                <Button>
                    id: "join"
                    center: "(0.0, 0.35)"
                    scale: "(0.1, 0.1)"
                    primary: "GRAY"
                    secondary: "DARKGRAY"
                    <Label>
                        text: "Connect"
                    </Label>
                </Button>
                <Button>
                    id: "back"
                    center: "(0.4, -0.4)"
                    scale: "(0.1, 0.1)"
                    primary: "GRAY"
                    secondary: "DARKGRAY"
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
                <TextInput>
                    id: "port"
                    primary: "WHITE"
                    secondary: "BLACK"
                    placeholder: "53000"
                    center: "(0.0, 0.0)"
                    scale: "(0.4, 0.1)"
                </TextInput>
                <Button>
                    id: "start"
                    center: "(0.0, 0.2)"
                    scale: "(0.2, 0.2)"
                    primary: "GRAY"
                    secondary: "DARKGRAY"
                    <Label> text: "Start server" </Label>
                </Button>
                <Button>
                    id: "back"
                    center: "(0.4, -0.4)"
                    scale: "(0.1, 0.1)"
                    primary: "GRAY"
                    secondary: "DARKGRAY"
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
                primary: "Color::from_rgba(0, 0, 0, 0)"
                <Label> text: "Work in progress..." primary: "Color::from_rgba(255, 255, 255, 50)" </Label>
                <Button>
                    id: "back"
                    center: "(0.4, -0.4)"
                    scale: "(0.1, 0.1)"
                    primary: "GRAY"
                    secondary: "DARKGRAY"
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