use crate::Color;
use std::collections::BTreeMap;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Palette {
    colors: BTreeMap<char, Color>,
}

impl Palette {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_color(&mut self, ch: char, color: Color) -> &mut Self {
        self.colors.insert(ch, color);
        self
    }

    pub fn get_color(&self, ch: char) -> Option<Color> {
        self.colors.get(&ch).copied()
    }
}
