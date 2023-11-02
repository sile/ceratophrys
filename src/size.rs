use crate::Position;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub const EMPTY: Self = Self::new(0, 0);

    // TODO: rename
    pub const fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    pub const fn square(size: u16) -> Self {
        Self::new(size, size)
    }

    pub const fn is_empty(self) -> bool {
        self.width == 0 || self.height == 0
    }

    pub fn area(self) -> u32 {
        u32::from(self.width) * u32::from(self.height)
    }

    pub fn contains(self, position: Position) -> bool {
        if position.x < 0 || position.y < 0 {
            return false;
        }
        (0..self.height).contains(&(position.y as u16))
            && (0..self.width).contains(&(position.x as u16))
    }

    pub fn max(self, other: Self) -> Self {
        Self::new(self.width.max(other.width), self.height.max(other.height))
    }

    pub fn min(self, other: Self) -> Self {
        Self::new(self.width.min(other.width), self.height.min(other.height))
    }

    pub fn positions(self) -> impl Iterator<Item = Position> {
        let w = i16::try_from(self.width).expect("width is too large");
        let h = i16::try_from(self.height).expect("height is too large");
        (0..h).flat_map(move |y| (0..w).map(move |x| Position::xy(x, y)))
    }

    pub fn edge_positions(self) -> impl Iterator<Item = Position> {
        fn if_iter(
            cond: bool,
            iter: impl Iterator<Item = Position>,
        ) -> impl Iterator<Item = Position> {
            cond.then_some(iter).into_iter().flatten()
        }

        let w = i16::try_from(self.width).expect("width is too large");
        let h = i16::try_from(self.height).expect("height is too large");

        let iter0 = if_iter(h > 0, (0..w).map(move |x| Position::xy(x, 0)));
        let iter1 = if_iter(h > 1, (0..w).map(move |x| Position::xy(x, h - 1)));
        let iter2 = if_iter(w > 0, (1..h - 1).map(move |y| Position::xy(0, y)));
        let iter3 = if_iter(w > 1, (1..h - 1).map(move |y| Position::xy(w - 1, y)));

        iter0.chain(iter1).chain(iter2).chain(iter3)
    }
}

impl FromIterator<Position> for Size {
    fn from_iter<T: IntoIterator<Item = Position>>(iter: T) -> Self {
        let mut min = Position::MAX;
        let mut max = Position::MIN;

        for position in iter {
            min = min.min(position);
            max = max.max(position);
        }

        if max < min {
            Self::EMPTY
        } else {
            let size = max - min;
            Self::new(size.x as u16 + 1, size.y as u16 + 1)
        }
    }
}
