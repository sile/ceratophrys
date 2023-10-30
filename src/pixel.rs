use crate::{Color, Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pixel {
    pub position: Point,
    pub color: Color,
}

impl Pixel {
    pub const fn new(position: Point, color: Color) -> Self {
        Self { position, color }
    }

    pub fn map_position<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Point) -> Point,
    {
        self.position = f(self.position);
        self
    }
}
