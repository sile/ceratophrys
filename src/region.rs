use crate::{Position, Size};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Region {
    pub start: Position,
    pub size: Size,
}

impl Region {
    pub const fn new(start: Position, size: Size) -> Self {
        Self { start, size }
    }
}
