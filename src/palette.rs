use crate::Color;
use std::collections::BTreeMap;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Palette {
    colors: BTreeMap<char, Color>,
    default_color: Color,
}

impl Palette {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn color(mut self, ch: char, color: Color) -> Self {
        self.colors.insert(ch, color);
        self
    }

    pub fn rgb_hex(self, ch: char, hex: u32) -> Self {
        self.color(ch, Color::rgb_hex(hex))
    }

    pub fn rgba_hex(self, ch: char, hex: u32) -> Self {
        self.color(ch, Color::rgba_hex(hex))
    }

    pub fn get_color(&self, ch: char) -> Option<Color> {
        self.colors.get(&ch).copied()
    }

    pub fn default_color(mut self, color: Color) -> Self {
        self.default_color = color;
        self
    }

    pub fn get_default_color(&self) -> Color {
        self.default_color
    }
}
