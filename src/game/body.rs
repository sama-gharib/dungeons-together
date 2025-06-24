use macroquad::prelude::*;

use auto_with::with;

#[derive(Debug, Clone)]
pub struct Body {
    pub position: Vec2,
    pub size: Vec2,
    pub velocity: Vec2,
    pub friction_factor: f32
}

impl Default for Body {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ONE * 100.0,
            velocity: Vec2::ZERO,
            friction_factor: 1.0
        }
    }
}

impl Body {
    
    with!(friction_factor : f32);
    
    pub fn slide(&mut self) {
        self.position += self.velocity;
        self.velocity *= self.friction_factor;
    }
    
    pub fn impulse(&mut self, i: Vec2) {
        self.velocity += i;
    }
    
    pub fn position(&self) -> Vec2 { self.position }
    pub fn size(&self) -> Vec2 { self.size }
}