use crate::{Canvas, Color, Point, Render, Size};

#[derive(Debug, Default, Clone, Copy)]
pub struct Rectangle {
    size: Size,
    color: Color,
    fill: bool,
}

impl Rectangle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn fill(mut self, fill: bool) -> Self {
        self.fill = fill;
        self
    }
}

impl Render for Rectangle {
    fn render(&self, point: Point, canvas: &mut Canvas) {
        if self.fill {
            for p in self.size.points() {
                canvas.set_pixel(point + p, self.color);
            }
        } else if !self.size.is_empty() {
            for x in 0..self.size.width {
                canvas.set_pixel(point.move_x_unsigned(x), self.color);
                canvas.set_pixel(point.move_xy_unsigned(x, self.size.height - 1), self.color);
            }
            for y in 0..self.size.height {
                canvas.set_pixel(point.move_y_unsigned(y), self.color);
                canvas.set_pixel(point.move_xy_unsigned(self.size.width - 1, y), self.color);
            }
        }
    }
}
