use macroquad::prelude::*;
use auto_with::with;

use crate::utils::{ Dynamic, Drawable, Controlable };

use super::subject::*;
use super::object::*;
use super::body::*;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
    Around
}

pub struct Collider<'a> {
    with: &'a GameComponent,
    from: Direction
}

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
        self.body.slide();
    }
}

impl Controlable for GameComponent {
    fn handle_events(&mut self) -> bool {
        let r = self.variant.handle_events();
        if let GameComponentVariant::Subject (subject) = &self.variant {
            self.body.impulse(subject.slide());
        }
        
        r
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
    
    pub fn collisions<'a>(&mut self, others: impl Iterator<Item=&'a GameComponent>, only_check: bool) -> Vec<Collider<'a>> {
        let mut collided_with = Vec::<Collider>::new();
        
        for other in others {
            let me = self.body();
            let you = other.body();
            
            if me.position.x + me.size.x + me.velocity.x > you.position.x + you.velocity.x
            && me.position.y + me.size.y + me.velocity.y > you.position.y + you.velocity.y
            && me.position.x + me.velocity.x < you.position.x + you.size.x + you.velocity.x
            && me.position.y + me.velocity.y < you.position.y + you.size.y + you.velocity.y {
                // There is a collision
                let dir = if you.position.x > me.position.x + me.size.x {
                    Direction::Right
                } else if you.position.x + you.size.x < me.position.x {
                    Direction::Left
                } else if you.position.y > me.position.y + me.size.y {
                    Direction::Down
                } else if you.position.y + you.size.y < me.position.y {
                    Direction::Up
                } else {
                    Direction::Around
                };
                
                collided_with.push(Collider { with: other, from: dir });
                
                if !only_check {
                    self.body.velocity = match dir {
                        Direction::Up => vec2(me.velocity.x, 0.0),
                        Direction::Left => vec2(0.0, me.velocity.y),
                        Direction::Down => vec2(me.velocity.x, 0.0),
                        Direction::Right => vec2(0.0, me.velocity.y),
                        Direction::Around => {
                            me.velocity
                        }
                    };
                }
            }
        }
        
        collided_with
    }
    
    pub fn body(&self) -> Body { self.body.clone() }
}