use crate::{Color, Entity, Filter, Image};

#[derive(Debug, Clone, Copy)]
pub struct Silhouette;

impl Filter<Image> for Silhouette {
    fn filter(&self, mut target: Image) -> Image {
        for color in target.colors_mut() {
            if !color.is_transparent() {
                *color = Color::BLACK;
            }
        }
        target
    }
}

impl Filter<Entity> for Silhouette {
    fn filter(&self, mut target: Entity) -> Entity {
        target.image = self.filter(target.image);
        for child in &mut target.children {
            *child = self.filter(std::mem::take(child));
        }
        target
    }
}
