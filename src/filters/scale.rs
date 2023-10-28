use std::num::NonZeroU8;

use crate::{Canvas, Entity, Filter, Image};

#[derive(Debug, Clone, Copy)]
pub struct Scale(NonZeroU8);

impl Scale {
    pub const fn new(value: NonZeroU8) -> Self {
        Self(value)
    }
}

impl Filter<Image> for Scale {
    fn filter(&self, target: Image) -> Image {
        let scale = i16::from(self.0.get());
        let mut canvas = Canvas::new();
        for (point, color) in target.pixels() {
            let point = point * scale;
            for y in 0..scale {
                for x in 0..scale {
                    canvas.set_pixel(point.move_xy(x, y), color);
                }
            }
        }
        canvas.to_image()
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
