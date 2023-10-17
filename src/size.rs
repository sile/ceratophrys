use crate::Point;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub const fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    pub fn area(self) -> u32 {
        u32::from(self.width) * u32::from(self.height)
    }

    pub fn points(self) -> impl Iterator<Item = Point> {
        (0..self.height)
            .flat_map(move |y| (0..self.width).map(move |x| Point::new(x as i16, y as i16)))
    }

    pub const fn is_empty(self) -> bool {
        self.width == 0 || self.height == 0
    }
}
