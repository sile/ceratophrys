use crate::{Canvas, Color, Point, Render, Size};

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    size: Size,
    color: Color,
    fill: bool,
}

impl Rectangle {
    pub fn new(color: Color, size: Size) -> Self {
        Self {
            color,
            size,
            fill: false,
        }
    }

    pub fn fill(mut self) -> Self {
        self.fill = true;
        self
    }
}

impl Render for Rectangle {
    fn render(&self, point: Point, canvas: &mut Canvas) {
        if self.fill {
            for p in self.size.points() {
                canvas.draw_pixel(point + p, self.color);
            }
        } else if !self.size.is_empty() {
            let mut pixels = Vec::new();
            for x in 0..self.size.width {
                pixels.push((point.move_x_unsigned(x), self.color));
                pixels.push((point.move_xy_unsigned(x, self.size.height - 1), self.color));
            }
            for y in 0..self.size.height {
                pixels.push((point.move_y_unsigned(y), self.color));
                pixels.push((point.move_xy_unsigned(self.size.width - 1, y), self.color));
            }
            pixels.sort();
            pixels.dedup();

            for (point, color) in pixels {
                canvas.draw_pixel(point, color);
            }
        }
    }
}
