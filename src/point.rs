use std::{
    cmp::Ordering,
    ops::{Add, Mul, Sub},
};

// TODO(?): s/Point/Position/
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

impl Point {
    pub const ORIGIN: Self = Self::new(0, 0);
    pub const MIN: Self = Self::new(i16::MIN, i16::MIN);
    pub const MAX: Self = Self::new(i16::MAX, i16::MAX);

    pub const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    pub const fn xy(x: i16, y: i16) -> Self {
        Self::new(x, y)
    }

    pub const fn yx(y: i16, x: i16) -> Self {
        Self::new(x, y)
    }

    pub const fn move_x(self, x: i16) -> Self {
        Self::new(self.x.saturating_add(x), self.y)
    }

    pub const fn move_x_unsigned(self, x: u16) -> Self {
        Self::new(self.x.saturating_add_unsigned(x), self.y)
    }

    pub const fn move_y(self, y: i16) -> Self {
        Self::new(self.x, self.y.saturating_add(y))
    }

    pub const fn move_y_unsigned(self, y: u16) -> Self {
        Self::new(self.x, self.y.saturating_add_unsigned(y))
    }

    pub const fn move_xy(self, x: i16, y: i16) -> Self {
        Self::new(self.x.saturating_add(x), self.y.saturating_add(y))
    }

    pub const fn move_xy_unsigned(self, x: u16, y: u16) -> Self {
        Self::new(
            self.x.saturating_add_unsigned(x),
            self.y.saturating_add_unsigned(y),
        )
    }

    pub fn min(self, other: Self) -> Self {
        Self::new(self.x.min(other.x), self.y.min(other.y))
    }

    pub fn max(self, other: Self) -> Self {
        Self::new(self.x.max(other.x), self.y.max(other.y))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, Self { x, y }: Self) -> Self::Output {
        Self::new(self.x.saturating_add(x), self.y.saturating_add(y))
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, Self { x, y }: Self) -> Self::Output {
        Self::new(self.x.saturating_sub(x), self.y.saturating_sub(y))
    }
}

impl Mul<i16> for Point {
    type Output = Self;

    fn mul(self, rhs: i16) -> Self::Output {
        Self::new(self.x.saturating_mul(rhs), self.y.saturating_mul(rhs))
    }
}
