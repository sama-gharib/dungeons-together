use macroquad::prelude::*;

use auto_with::with;

use crate::utils::{Controlable, Dynamic};

use super::keys::*;

#[derive(Debug, Clone)]
pub enum Brain {
    Player (KeyBinding),
    Monster { target: Vec2 }
}

#[derive(Debug, Clone)]
pub struct GameSubject {
    speed: f32,
    
    slide: Vec2,
    orientation: Vec2,
    
    brain: Brain
}

impl Controlable for GameSubject {
    fn handle_events(&mut self) -> bool {
        if let Brain::Player(controls) = &self.brain {
            self.slide = controls.slide.get_vec() * self.speed * get_frame_time();
            self.orientation = controls.look.get_vec();
                    
            self.slide != Vec2::ZERO || self.orientation != Vec2::ZERO
        } else {
            false
        }
    }
}

impl Dynamic for GameSubject {
    fn update(&mut self) {
        if let Brain::Monster { target } = &self.brain {
            todo!()
        }
    }
}


impl Default for GameSubject {
    fn default() -> Self {
        Self {
            speed: 50.0,
            slide: Vec2::ZERO,
            orientation: Vec2::ZERO,
            brain: Brain::Player ( KeyBinding::default() )
        }
    }
}

impl GameSubject {
    with! { brain: Brain }
    
    pub fn slide(&self) -> Vec2 { self.slide }
}