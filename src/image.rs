use crate::{Color, Pixel, Point, Size};
use std::collections::BTreeMap;

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

    pub fn from_text(palette: impl IntoIterator<Item = (char, Color)>, text: &str) -> Self {
        let mut size = Size::EMPTY;
        for line in text.lines() {
            size.height += 1;
            size.width = size.width.max(line.chars().count() as u16);
        }

        let palette = BTreeMap::from_iter(palette);
        let mut colors = vec![Color::TRANSPARENT; size.area() as usize];
        for (y, line) in text.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let i = size.width as usize * y + x;
                colors[i] = palette.get(&ch).copied().unwrap_or(Color::TRANSPARENT);
            }
        }

        Self::new(size, colors)
    }

    pub fn to_text(&self) -> String {
        let mut colors = " 0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
        let mut color_to_char = BTreeMap::new();
        let mut text = String::new();

        for (i, pixel) in self.pixels().enumerate() {
            let ch = color_to_char
                .entry(pixel.color)
                .or_insert_with(|| colors.next().unwrap_or('?'));
            text.push(*ch);
            if (i + 1) % self.size.width as usize == 0 {
                text.push('\n');
            }
        }

        text
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn colors(&self) -> &[Color] {
        &self.pixels
    }

    pub fn pixels(&self) -> impl '_ + Iterator<Item = Pixel> {
        self.size
            .points()
            .zip(self.pixels.iter().copied())
            .map(|(position, color)| Pixel::new(position, color))
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

impl FromIterator<Pixel> for Image {
    fn from_iter<T: IntoIterator<Item = Pixel>>(iter: T) -> Self {
        let mut pixels = Vec::new();
        let mut position_min = Point::MAX;
        let mut position_max = Point::MIN;
        for pixel in iter {
            pixels.push(pixel);
            position_min = position_min.min(pixel.position);
            position_max = position_max.max(pixel.position);
        }
        if pixels.is_empty() {
            Self::default()
        } else {
            let size = Size::new(
                position_max.x.saturating_sub(position_min.x) as u16 + 1,
                position_max.y.saturating_sub(position_min.y) as u16 + 1,
            );
            let mut colors = vec![Color::TRANSPARENT; size.area() as usize];
            for pixel in pixels {
                let position = pixel.position - position_min;
                let i = size.width as usize * position.y as usize + position.x as usize;
                colors[i] = pixel.color.alpha_blend(colors[i]);
            }
            Self::new(size, colors)
        }
    }
}
