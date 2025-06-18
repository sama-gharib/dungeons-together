use crate::utils::Drawable;

pub enum GameObject {
    CheckPoint { priority: usize },
    Wall,
    Projectile
}

impl Drawable for GameObject {
    fn draw(&self) {
        todo!()
    }
}