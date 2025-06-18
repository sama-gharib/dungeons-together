use macroquad::prelude::*;

pub struct DirectionKeys {
    pub up: KeyCode,
    pub left: KeyCode,
    pub down: KeyCode,
    pub right: KeyCode
}

impl DirectionKeys {
    pub fn get_vec(&self) -> Vec2 {
        let mut dir = Vec::new();
        for key in get_keys_down() {
            match key {
                key if key == self.up => dir.push(vec2(0.0, -1.0)),
                key if key == self.left => dir.push(vec2(-1.0, 0.0)),
                key if key == self.down => dir.push(vec2(0.0, 1.0)),
                key if key == self.right => dir.push(vec2(1.0, 0.0)),
                _ => {}
            }
        }
        
        dir
            .into_iter()
            .reduce(|x, y| x + y)
            .unwrap_or(Vec2::ZERO)
            .normalize_or(Vec2::ZERO)
    }
}

pub struct KeyBinding {
    pub slide: DirectionKeys,
    pub look: DirectionKeys,
    
    pub action: KeyCode
}

impl Default for KeyBinding {
    fn default() -> Self {
        Self {
            slide: DirectionKeys {
                up: KeyCode::Z,
                left: KeyCode::Q,
                down: KeyCode::S,
                right: KeyCode::D
            },
            look: DirectionKeys {
                up: KeyCode::Up,
                left: KeyCode::Left,
                down: KeyCode::Down,
                right: KeyCode::Right
            },
            action: KeyCode::Space
        }
    }
}