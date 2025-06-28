use macroquad::prelude::*;

use super::{component::GameComponent, keys::KeyBinding, map::Map};

#[derive(Copy, Clone, Debug, Default)]
pub struct Movement {
    pub velocity: Vec2,
    pub orientation: Vec2
}


#[derive(Debug, Clone)]
pub enum Controller {
    Player { controls: KeyBinding, speed: f32 },
    Monster,
    BrainDead,
}

impl Default for Controller {
    fn default() -> Self {
        Self::Player {
            controls: KeyBinding::default(),
            speed: 100.0
        }
    }
}

impl Controller {
    pub fn get_movement(&mut self, from: Vec2, world: &Map) -> Movement {
        let mut movement = Movement::default();
        match self {
            Self::Player { controls, speed } => {
                movement.velocity = controls.slide.get_vec() * *speed * get_frame_time();
                movement.orientation = controls.look.get_vec();
            },
            Self::Monster => {
                movement.velocity = vec2(10.0, 0.0);
            },
            Self::BrainDead => {}
        }
        
        movement
    }
}
