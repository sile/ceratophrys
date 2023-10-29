use crate::{Color, Image, Point, Size};

// TODO: Point

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

impl From<Rectangle> for Image {
    fn from(shape: Rectangle) -> Self {
        let pixels = Vec::new();
        // if shape.fill {
        //     pixels=shape.size.points().map(|p| shape.color).collect();
        //     for p in self.size.points() {
        //         canvas.draw_pixel(point + p, self.color);
        //     }
        // } else {
        // }
        Image::new(shape.size, pixels)
    }
}
