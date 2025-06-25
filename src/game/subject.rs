use macroquad::prelude::*;

use crate::utils::Controlable;

use super::keys::*;

#[derive(Debug, Clone)]
pub struct GameSubject {
    controls: KeyBinding,
    speed: f32,
    
    slide: Vec2,
    orientation: Vec2
}

impl Controlable for GameSubject {
    fn handle_events(&mut self) -> bool {
        self.slide = self.controls.slide.get_vec() * self.speed * get_frame_time();
        self.orientation = self.controls.look.get_vec();
                
        self.slide != Vec2::ZERO || self.orientation != Vec2::ZERO
    }
}

impl Default for GameSubject {
    fn default() -> Self {
        Self {
            controls: KeyBinding::default(),
            speed: 100.0,
            slide: Vec2::ZERO,
            orientation: Vec2::ZERO
        }
    }
}

impl GameSubject {
    pub fn slide(&self) -> Vec2 { self.slide }
}