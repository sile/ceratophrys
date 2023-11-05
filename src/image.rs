use crate::{filters::Filter, Color, Pixel, Position, Region, Size};
use std::collections::BTreeMap;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Image {
    pub name: Option<String>,
    pub pixels: BTreeMap<Position, Color>,
    pub children: Vec<Self>,
}

impl Image {
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

    pub fn offset(mut self, offset: Position) -> Self {
        self.pixels = self
            .pixels
            .into_iter()
            .map(|(position, color)| (position + offset, color))
            .collect();
        for child in &mut self.children {
            *child = std::mem::take(child).offset(offset);
        }
        self
    }

    pub fn get_region(&self) -> Region {
        let mut min = Position::MAX;
        let mut max = Position::MIN;
        for position in self.positions() {
            min = min.min(position);
            max = max.max(position);
        }
        if max < min {
            Region::new(Position::ORIGIN, Size::EMPTY)
        } else {
            let size = max - min;
            let size = Size::new(size.x as u16 + 1, size.y as u16 + 1);
            Region::new(min, size)
        }
    }

    // TODO: remove
    pub fn get_size(&self) -> Size {
        self.positions().collect()
    }

    pub fn get_top(&self) -> i16 {
        self.positions().map(|p| p.y).min().unwrap_or(0)
    }

    pub fn get_bottom(&self) -> i16 {
        self.positions().map(|p| p.y).max().unwrap_or(0)
    }

    pub fn get_left(&self) -> i16 {
        self.positions().map(|p| p.x).min().unwrap_or(0)
    }

    pub fn get_right(&self) -> i16 {
        self.positions().map(|p| p.x).max().unwrap_or(0)
    }

    pub fn get_color(&self, position: Position) -> Color {
        let mut color = self
            .pixels
            .get(&position)
            .copied()
            .unwrap_or(Color::TRANSPARENT);
        for child in &self.children {
            color = child.get_color(position).alpha_blend(color);
        }
        color
    }

    pub fn child(mut self, child: Self) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = Self>) -> Self {
        self.children.extend(children);
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

    pub fn positions(&self) -> impl '_ + Iterator<Item = Position> {
        self.iter().map(|p| p.position)
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

    pub fn filter<F: Filter>(mut self, filter: F) -> Self {
        filter.filter(&mut self);
        self
    }
}

impl IntoIterator for Image {
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

impl FromIterator<Pixel> for Image {
    fn from_iter<T: IntoIterator<Item = Pixel>>(iter: T) -> Self {
        let mut image = Self::new();
        image.extend(iter);
        image
    }
}

impl Extend<Pixel> for Image {
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
