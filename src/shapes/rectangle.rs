use crate::{Color, Pixel, Point, Size};

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub position: Point,
    pub size: Size,
    pub color: Color,
    pub fill: bool,
}

impl Rectangle {
    pub const fn new() -> Self {
        Self {
            position: Point::ORIGIN,
            size: Size::EMPTY,
            color: Color::BLACK,
            fill: false,
        }
    }

    pub const fn position(self, position: Point) -> Self {
        Self { position, ..self }
    }

    pub const fn size(self, size: Size) -> Self {
        Self { size, ..self }
    }

    pub const fn color(self, color: Color) -> Self {
        Self { color, ..self }
    }

    pub const fn fill(self) -> Self {
        Self { fill: true, ..self }
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for Rectangle {
    type Item = Pixel;
    type IntoIter = Box<dyn Iterator<Item = Pixel>>;

    fn into_iter(self) -> Self::IntoIter {
        if self.fill || self.size.is_empty() {
            return Box::new(
                self.size
                    .points()
                    .map(move |p| Pixel::new(self.position + p, self.color)),
            );
        }

        let pixel = Pixel::new(self.position, self.color);
        let mut pixels = Vec::new();
        for x in 0..self.size.width {
            pixels.push(pixel.map_position(|p| p.move_x_unsigned(x)));
            pixels.push(pixel.map_position(|p| p.move_xy_unsigned(x, self.size.height - 1)));
        }
        for y in 0..self.size.height {
            pixels.push(pixel.map_position(|p| p.move_y_unsigned(y)));
            pixels.push(pixel.map_position(|p| p.move_xy_unsigned(self.size.width - 1, y)));
        }
        pixels.sort();
        pixels.dedup();
        Box::new(pixels.into_iter())
    }
}
