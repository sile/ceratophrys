use crate::{Canvas, Color, Point, Render, Size};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Image {
    size: Size,
    pixels: Vec<Color>,
}

impl Image {
    pub fn new(size: Size, pixels: Vec<Color>) -> Option<Self> {
        (size.area() == pixels.len() as u32).then(|| Self { size, pixels })
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn pixels(&self) -> &[Color] {
        &self.pixels
    }

    pub fn rows(&self) -> impl '_ + DoubleEndedIterator<Item = &[Color]> {
        self.pixels.chunks(self.size.width as usize)
    }
}

impl Render for Image {
    fn render(&self, point: Point, canvas: &mut Canvas) {
        for (p, c) in self.size().points().zip(self.pixels.iter().copied()) {
            canvas.set_pixel(point + p, c);
        }
    }
}
