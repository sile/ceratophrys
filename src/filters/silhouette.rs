use crate::{Color, Filter, Image};

#[derive(Debug, Clone, Copy)]
pub struct Silhouette;

impl Filter for Silhouette {
    fn filter(&self, mut image: Image) -> Image {
        for color in image.pixels.values_mut() {
            if !color.is_transparent() {
                *color = Color::BLACK;
            }
        }
        for child in &mut image.children {
            *child = self.filter(std::mem::take(child));
        }
        image
    }
}
