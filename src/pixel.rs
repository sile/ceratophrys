use crate::{Color, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pixel {
    pub position: Position,
    pub color: Color,
}

impl Pixel {
    pub const fn new(position: Position, color: Color) -> Self {
        Self { position, color }
    }

    pub fn map_position<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Position) -> Position,
    {
        self.position = f(self.position);
        self
    }
}
