use macroquad::prelude::*;
use auto_with::with;

use crate::utils::{ Dynamic, Drawable, Controlable };

use super::controller::Controller;
use super::controller::Movement;
use super::keys::KeyBinding;
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
pub struct GameComponent {
    pub body: Body,
    pub object: GameObject,
    pub controller: Controller
}

impl Drawable for GameComponent {
    fn draw(&self) {
        let color = match &self.object {
            GameObject::Player => BLUE,
            GameObject::CheckPoint {..} => GREEN,
            GameObject::Wall => RED,
            GameObject::Projectile => YELLOW
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
        let movement = self.controller.get_movement();
        
        self.body.impulse(movement.velocity);
        
        movement.velocity != Vec2::ZERO && movement.orientation != Vec2::ZERO
    }
}

impl From<GameObject> for GameComponent {
    fn from(o: GameObject) -> Self {
        Self {
            body: match o {
                GameObject::Wall => Body::default(),
                GameObject::Projectile => todo!(),
                GameObject::CheckPoint { .. } => todo!(),
                GameObject::Player => Body::default()
                    .with_friction_factor(0.9)
            },
            controller: if let GameObject::Player = o {
                Controller::Player { controls: KeyBinding::default(), speed: 100.0 }
            } else {
                Controller::BrainDead
            },
            object: o,
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