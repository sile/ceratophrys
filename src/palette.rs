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

    pub fn get_color(&self, ch: char) -> Color {
        self.colors.get(&ch).copied().unwrap_or(self.default_color)
    }

    pub fn default_color(mut self, color: Color) -> Self {
        self.default_color = color;
        self
    }

    pub fn get_default_color(&self) -> Color {
        self.default_color
    }
}
