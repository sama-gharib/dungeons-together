use macroquad::prelude::*;
use auto_with::with;

use crate::utils::{ Dynamic, Drawable, Controlable };

use super::subject::*;
use super::object::*;
use super::body::*;

#[derive(Debug, Clone)]
pub enum GameComponentVariant {
    Subject (GameSubject),
    Object (GameObject)
}

impl Controlable for GameComponentVariant {
    fn handle_events(&mut self) -> bool {
        match self {
            Self::Subject(subject) => subject.handle_events(),
            Self::Object(_) => { false }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameComponent {
    pub body: Body,
    pub variant: GameComponentVariant
}

impl Drawable for GameComponent {
    fn draw(&self) {
        let color = match &self.variant {
            GameComponentVariant::Subject(_subject) => BLUE,
            GameComponentVariant::Object(_object) => RED
        };
        
        draw_rectangle(
            self.body.position().x,
            self.body.position().y,
            self.body.size().x,
            self.body.size().y,
            color
        );
    }
}

impl Dynamic for GameComponent {
    fn update(&mut self) {
        if let GameComponentVariant::Subject (subject) = &self.variant {
            self.body.impulse(subject.slide());
        }
        self.body.slide();
    }
}

impl Controlable for GameComponent {
    fn handle_events(&mut self) -> bool {
        self.variant.handle_events()
    }
}

impl From<GameComponentVariant> for GameComponent {
    fn from(s: GameComponentVariant) -> Self {
        Self {
            body: match s {
                GameComponentVariant::Object(_) => Body::default(),
                GameComponentVariant::Subject(_) => Body::default()
                    .with_friction_factor(0.9)
            },
            variant: s
        }
    }
}

impl GameComponent {
    with!{ body: Body }
    
    pub fn body(&self) -> Body { self.body.clone() }
}