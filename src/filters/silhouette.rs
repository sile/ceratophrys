use crate::{filters::Filter, Color, Image};

#[derive(Debug, Clone, Copy)]
pub struct Silhouette;

impl Filter for Silhouette {
    fn filter(&self, image: &mut Image) {
        for color in image.pixels.values_mut() {
            if !color.is_transparent() {
                *color = Color::BLACK;
            }
        }
        for child in &mut image.children {
            self.filter(child);
        }
    }
}
