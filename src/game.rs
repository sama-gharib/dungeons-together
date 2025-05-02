use macroquad::prelude::*;

pub enum Direction {
    Up,
    Left,
    Down,
    Right
}

pub trait Drawable {
    fn draw(&self);
}

pub trait Dynamic {
    fn update(&mut self);
}

pub trait Physical {
    fn get_position(&self) -> Vec2;
    fn get_size(&self)     -> Vec2;
    fn get_speed(&self)    -> Vec2;
    
    fn collides(elf: &impl Physical, other: &impl Physical) -> Option<Direction> {
        let me  = Body::from(elf);
        let you = Body::from(other);
        
        if me.position.x + me.size.x + me.speed.x > you.position.x + you.speed.x &&
           me.position.y + me.size.y + me.speed.y > you.position.y + you.speed.y &&
           me.position.x + me.speed.x < you.position.x + you.size.x + you.speed.x &&
           me.position.y + me.speed.y < you.position.y + you.size.y + you.speed.y {
                Some(
                    if me.position.x + me.size.x < you.position.x {
                        Direction::Right
                    } else if me.position.y + me.size.y < you.position.y {
                        Direction::Down
                    } else if me.position.x > you.position.x + you.size.x {
                        Direction::Left
                    } else {
                        Direction::Up
                    }
                )   
        } else {
            None
        }
    }
}

pub struct Body {
    pub position: Vec2,
    pub size: Vec2,
    pub speed: Vec2,
    pub future_speed: Option<Vec2>
}

impl Dynamic for Body {
    fn update(&mut self) {
        if let Some(s) = self.future_speed {
            self.speed = s;
        }
        self.position += self.speed;
    }
}

impl Drawable for Body {
    fn draw(&self) {
        draw_rectangle(self.position.x, self.position.y, self.size.x, self.size.y, WHITE);
    }
}


impl Body {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        Self {
            position,
            size,
            speed: Vec2::ZERO,
            future_speed: None
        }
    }
    
    fn from(p: &impl Physical) -> Self {
        Self {
            position: p.get_position(),
            size: p.get_size(),
            speed: p.get_speed(),
            future_speed: None
        }
    }
}