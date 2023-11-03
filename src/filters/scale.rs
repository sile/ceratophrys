use crate::{Filter, Image, Size};
use std::num::NonZeroU8;

#[derive(Debug, Clone, Copy)]
pub struct Scale(NonZeroU8);

impl Scale {
    pub const fn new(value: NonZeroU8) -> Self {
        Self(value)
    }
}

impl Filter for Scale {
    fn filter(&self, mut image: Image) -> Image {
        let scale = self.0.get();
        image.pixels = image
            .pixels
            .into_iter()
            .flat_map(|(position, color)| {
                Size::square(u16::from(scale))
                    .positions()
                    .map(move |offset| (position * i16::from(scale) + offset, color))
            })
            .collect();
        for child in &mut image.children {
            *child = self.filter(std::mem::take(child));
        }
        image
    }
}
