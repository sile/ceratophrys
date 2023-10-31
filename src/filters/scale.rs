use crate::{Entity, Filter, Image, Pixel, Size};
use std::num::NonZeroU8;

#[derive(Debug, Clone, Copy)]
pub struct Scale(NonZeroU8);

impl Scale {
    pub const fn new(value: NonZeroU8) -> Self {
        Self(value)
    }
}

impl Filter<Image> for Scale {
    fn filter(&self, target: Image) -> Image {
        let scale = self.0.get();
        target
            .pixels()
            .flat_map(|Pixel { position, color }| {
                Size::square(u16::from(scale))
                    .positions()
                    .map(move |offset| Pixel::new(position * i16::from(scale) + offset, color))
            })
            .collect()
    }
}

impl Filter<Entity> for Scale {
    fn filter(&self, mut target: Entity) -> Entity {
        let scale = i16::from(self.0.get());
        target.image = self.filter(target.image);
        for child in &mut target.children {
            *child = self.filter(std::mem::take(child));
            child.offset = child.offset * scale;
        }
        target
    }
}
