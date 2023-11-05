use crate::{Position, Size};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Region {
    pub start: Position,
    pub size: Size,
}

impl Region {
    pub const fn new(start: Position, size: Size) -> Self {
        Self { start, size }
    }

    pub fn is_empty(self) -> bool {
        self.size.is_empty()
    }

    pub const fn top(self) -> i16 {
        self.start.y
    }

    pub const fn bottom(self) -> i16 {
        self.start.y + self.size.height as i16 - 1
    }

    pub const fn left(self) -> i16 {
        self.start.x
    }

    pub const fn right(self) -> i16 {
        self.start.x + self.size.width as i16 - 1
    }

    pub const fn top_left(self) -> Position {
        self.start
    }

    pub const fn top_right(self) -> Position {
        Position::xy(self.right(), self.top())
    }

    pub const fn bottom_left(self) -> Position {
        Position::xy(self.left(), self.bottom())
    }

    pub const fn bottom_right(self) -> Position {
        Position::xy(self.right(), self.bottom())
    }

    pub fn positions(self) -> impl Iterator<Item = Position> {
        self.size
            .positions()
            .map(move |position| position + self.start)
    }
}

impl FromIterator<Position> for Region {
    fn from_iter<T: IntoIterator<Item = Position>>(iter: T) -> Self {
        let mut min = Position::MAX;
        let mut max = Position::MIN;
        for position in iter {
            min = min.min(position);
            max = max.max(position);
        }
        if max < min {
            Self::new(Position::ORIGIN, Size::EMPTY)
        } else {
            let size = max - min;
            let size = Size::new(size.x as u16 + 1, size.y as u16 + 1);
            Self::new(min, size)
        }
    }
}
