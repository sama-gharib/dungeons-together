use crate::utils::Drawable;

#[derive(Debug, Clone)]
pub enum GameObject {
    Player,
    CheckPoint { priority: usize },
    Wall,
    Projectile
}

impl Drawable for GameObject {
    fn draw(&self) {
        todo!()
    }
}