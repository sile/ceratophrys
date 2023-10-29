use crate::{Color, Point, Size};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Image {
    size: Size,
    pixels: Vec<Color>,
}

impl Image {
    pub fn new(size: Size, mut pixels: Vec<Color>) -> Self {
        pixels.resize(size.area() as usize, Color::default());
        Self { size, pixels }
    }

    // TODO: from_text(), to_text()

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn colors(&self) -> &[Color] {
        &self.pixels
    }

    pub fn pixels(&self) -> impl '_ + Iterator<Item = (Point, Color)> {
        self.size.points().zip(self.pixels.iter().copied())
    }

    pub fn rows(&self) -> impl '_ + DoubleEndedIterator<Item = &[Color]> {
        self.pixels.chunks(self.size.width as usize)
    }

    pub fn get_color(&self, point: Point) -> Option<Color> {
        self.pixels
            .get(self.size.width as usize * point.y as usize + point.x as usize)
            .copied()
    }
}
