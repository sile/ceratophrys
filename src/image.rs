use crate::{Color, Filter, Pixel, Position, Size};
use std::collections::BTreeMap;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Image2 {
    pub name: Option<String>,
    pub pixels: BTreeMap<Position, Color>,
    pub children: Vec<Self>,
}

impl Image2 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_text(palette: impl IntoIterator<Item = (char, Color)>, text: &str) -> Self {
        let palette = BTreeMap::from_iter(palette);
        text.lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, ch)| (Position::xy(x as i16, y as i16), ch))
            })
            .map(move |(position, ch)| {
                let color = palette.get(&ch).copied().unwrap_or(Color::TRANSPARENT);
                Pixel::new(position, color)
            })
            .collect()
    }

    pub fn to_text(&self) -> String {
        let mut colors = " 0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
        let mut color_to_char = BTreeMap::new();
        let mut text = String::new();

        let (size, pixel_colors) = self.to_size_and_colors();
        for (i, color) in pixel_colors.into_iter().enumerate() {
            let ch = color_to_char
                .entry(color)
                .or_insert_with(|| colors.next().unwrap_or('?'));
            text.push(*ch);
            if (i + 1) % size.width as usize == 0 {
                text.push('\n');
            }
        }

        text
    }

    pub fn name(self, name: &str) -> Self {
        Self {
            name: Some(name.to_owned()),
            ..self
        }
    }

    pub fn child(mut self, child: Self) -> Self {
        self.children.push(child);
        self
    }

    pub fn to_size_and_colors(&self) -> (Size, Vec<Color>) {
        let size = Size::from_iter(self.iter().map(|p| p.position));

        let mut colors = vec![Color::TRANSPARENT; size.area() as usize];
        for pixel in self.iter() {
            let i = size.width as usize * pixel.position.y as usize + pixel.position.x as usize;
            colors[i] = pixel.color.alpha_blend(colors[i]);
        }

        (size, colors)
    }

    pub fn iter(&self) -> impl '_ + Iterator<Item = Pixel> {
        self.pixels
            .iter()
            .map(|(&position, &color)| Pixel::new(position, color))
            .chain(
                Box::new(self.children.iter().flat_map(|child| child.iter()))
                    as Box<dyn Iterator<Item = Pixel>>,
            )
    }
}

impl IntoIterator for Image2 {
    type Item = Pixel;
    type IntoIter = Box<dyn Iterator<Item = Pixel>>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(
            self.pixels
                .into_iter()
                .map(|(position, color)| Pixel::new(position, color))
                .chain(
                    self.children
                        .into_iter()
                        .flat_map(|child| child.into_iter()),
                ),
        )
    }
}

impl FromIterator<Pixel> for Image2 {
    fn from_iter<T: IntoIterator<Item = Pixel>>(iter: T) -> Self {
        let mut image = Self::new();
        image.extend(iter);
        image
    }
}

impl Extend<Pixel> for Image2 {
    fn extend<T: IntoIterator<Item = Pixel>>(&mut self, iter: T) {
        for pixel in iter {
            self.pixels
                .entry(pixel.position)
                .and_modify(|color| {
                    *color = pixel.color.alpha_blend(*color);
                })
                .or_insert(pixel.color);
        }
    }
}

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

    pub fn colors_mut(&mut self) -> &mut [Color] {
        &mut self.pixels
    }

    pub fn pixels(&self) -> impl '_ + Iterator<Item = Pixel> {
        self.size
            .positions()
            .zip(self.pixels.iter().copied())
            .map(|(position, color)| Pixel::new(position, color))
    }

    pub fn rows(&self) -> impl '_ + DoubleEndedIterator<Item = &[Color]> {
        self.pixels.chunks(self.size.width as usize)
    }

    pub fn get_color(&self, position: Position) -> Option<Color> {
        self.pixels
            .get(self.size.width as usize * position.y as usize + position.x as usize)
            .copied()
    }

    pub fn filter<F: Filter<Image>>(self, filter: F) -> Self {
        filter.filter(self)
    }
}

impl FromIterator<Pixel> for Image {
    fn from_iter<T: IntoIterator<Item = Pixel>>(iter: T) -> Self {
        let mut pixels = Vec::new();
        let mut position_min = Position::MAX;
        let mut position_max = Position::MIN;
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
