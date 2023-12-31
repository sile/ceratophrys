use std::{
    cmp::Ordering,
    ops::{Add, Mul, Sub},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}

impl Position {
    pub const ORIGIN: Self = Self::xy(0, 0);
    pub const MIN: Self = Self::xy(i16::MIN, i16::MIN);
    pub const MAX: Self = Self::xy(i16::MAX, i16::MAX);

    pub const fn xy(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    pub const fn yx(y: i16, x: i16) -> Self {
        Self { x, y }
    }

    // TODO: remove
    pub const fn move_x(self, x: i16) -> Self {
        Self::xy(self.x.saturating_add(x), self.y)
    }

    pub const fn move_x_unsigned(self, x: u16) -> Self {
        Self::xy(self.x.saturating_add_unsigned(x), self.y)
    }

    pub const fn move_y(self, y: i16) -> Self {
        Self::xy(self.x, self.y.saturating_add(y))
    }

    pub const fn move_y_unsigned(self, y: u16) -> Self {
        Self::xy(self.x, self.y.saturating_add_unsigned(y))
    }

    pub const fn move_xy(self, x: i16, y: i16) -> Self {
        Self::xy(self.x.saturating_add(x), self.y.saturating_add(y))
    }

    pub const fn move_xy_unsigned(self, x: u16, y: u16) -> Self {
        Self::xy(
            self.x.saturating_add_unsigned(x),
            self.y.saturating_add_unsigned(y),
        )
    }

    pub fn min(self, other: Self) -> Self {
        Self::xy(self.x.min(other.x), self.y.min(other.y))
    }

    pub fn max(self, other: Self) -> Self {
        Self::xy(self.x.max(other.x), self.y.max(other.y))
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, Self { x, y }: Self) -> Self::Output {
        Self::xy(
            self.x.checked_add(x).expect("addition overflow"),
            self.y.checked_add(y).expect("addition overflow"),
        )
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, Self { x, y }: Self) -> Self::Output {
        Self::xy(
            self.x.checked_sub(x).expect("subtraction overflow"),
            self.y.checked_sub(y).expect("subtraction overflow"),
        )
    }
}

impl Mul<i16> for Position {
    type Output = Self;

    fn mul(self, rhs: i16) -> Self::Output {
        Self::xy(self.x.saturating_mul(rhs), self.y.saturating_mul(rhs))
    }
}
