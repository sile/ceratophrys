use crate::Point;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub const EMPTY: Self = Self::new(0, 0);

    pub const fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    pub const fn square(size: u16) -> Self {
        Self::new(size, size)
    }

    pub fn contains(self, point: Point) -> bool {
        if point.x < 0 || point.y < 0 {
            return false;
        }
        (0..self.height).contains(&(point.y as u16)) && (0..self.width).contains(&(point.x as u16))
    }

    pub fn area(self) -> u32 {
        u32::from(self.width) * u32::from(self.height)
    }

    pub fn points(self) -> impl Iterator<Item = Point> {
        (0..self.height)
            .flat_map(move |y| (0..self.width).map(move |x| Point::new(x as i16, y as i16)))
    }

    pub fn max(self, other: Self) -> Self {
        Self::new(self.width.max(other.width), self.height.max(other.height))
    }

    pub fn min(self, other: Self) -> Self {
        Self::new(self.width.min(other.width), self.height.min(other.height))
    }

    pub fn edge_points(self) -> impl Iterator<Item = Point> {
        // TODO: Avoid allocation
        let mut points = Vec::new();
        if self.is_empty() {
            return points.into_iter();
        }

        points.extend((0..self.width).map(move |x| Point::new(x as i16, 0)));
        points.extend((0..self.width).map(move |x| Point::new(x as i16, self.height as i16 - 1)));
        points.extend((0..self.height).map(move |y| Point::new(0, y as i16)));
        points.extend((0..self.height).map(move |y| Point::new(self.width as i16 - 1, y as i16)));
        points.sort();
        points.dedup();
        points.into_iter()
    }

    pub const fn is_empty(self) -> bool {
        self.width == 0 || self.height == 0
    }
}
