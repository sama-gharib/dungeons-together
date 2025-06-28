use macroquad::prelude::*;

use super::keys::KeyBinding;

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
    pub fn get_movement(&mut self) -> Movement {
        let mut movement = Movement::default();
        match self {
            Self::Player { controls, speed } => {
                movement.velocity = controls.slide.get_vec() * *speed * get_frame_time();
                movement.orientation = controls.look.get_vec();
            },
            Self::Monster => {
                todo!()
            },
            Self::BrainDead => {}
        }
        
        movement
    }
}
