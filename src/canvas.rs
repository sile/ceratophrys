use crate::{Color, Image, Point, Size};
use std::collections::BTreeMap;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Canvas {
    pixels: BTreeMap<Point, Color>,
    start_point: Point,
    end_point: Point,
    background_color: Color, // TODO: delete
}

impl Canvas {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pixels(&self) -> impl '_ + Iterator<Item = (Point, Color)> {
        self.pixels.iter().map(|(&point, &color)| (point, color))
    }

    pub fn draw_pixel(&mut self, point: Point, color: Color) {
        self.start_point.x = self.start_point.x.min(point.x);
        self.start_point.y = self.start_point.y.min(point.y);

        self.end_point.x = self.end_point.x.max(point.x + 1);
        self.end_point.y = self.end_point.y.max(point.y + 1);

        let color = self
            .pixels
            .get(&point)
            .map_or(color, |&old_color| color.alpha_blend(old_color));
        self.pixels.insert(point, color);
    }

    pub fn set_pixel(&mut self, point: Point, color: Color) {
        self.start_point.x = self.start_point.x.min(point.x);
        self.start_point.y = self.start_point.y.min(point.y);

        self.end_point.x = self.end_point.x.max(point.x + 1);
        self.end_point.y = self.end_point.y.max(point.y + 1);

        self.pixels.insert(point, color);
    }

    pub fn erase_pixel(&mut self, point: Point) {
        if self.pixels.remove(&point).is_none() {
            return;
        }

        if self.start_point.x == point.x {
            self.start_point.x = self.pixels.keys().map(|p| p.x).min().unwrap_or_default();
        }
        if self.start_point.y == point.y {
            self.start_point.y = self.pixels.keys().map(|p| p.y).min().unwrap_or_default();
        }

        if self.end_point.x == point.x + 1 {
            self.end_point.x = self
                .pixels
                .keys()
                .map(|p| p.x + 1)
                .max()
                .unwrap_or_default();
        }
        if self.end_point.y == point.y + 1 {
            self.end_point.y = self
                .pixels
                .keys()
                .map(|p| p.y + 1)
                .max()
                .unwrap_or_default();
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn start_point(&self) -> Point {
        self.start_point
    }

    pub fn end_point(&self) -> Point {
        self.end_point
    }

    pub fn size(&self) -> Size {
        Size::new(
            self.end_point.x.saturating_sub(self.start_point.x) as u16,
            self.end_point.y.saturating_sub(self.start_point.y) as u16,
        )
    }

    pub fn to_image(&self) -> Image {
        let start_point = self.start_point();
        let size = self.size();
        let mut pixels = vec![self.background_color; size.area() as usize];
        for (point, color) in self.pixels() {
            let point = point - start_point;
            let index = point.y as usize * size.width as usize + point.x as usize;
            pixels[index] = color.alpha_blend(pixels[index]);
        }
        Image::new(size, pixels)
    }
}
