use crate::{Canvas, Point, Render};
use std::num::NonZeroU8;

#[derive(Debug, Clone, Copy)]
pub struct Scale<T> {
    scale: NonZeroU8,
    image: T,
}

impl<T> Scale<T> {
    pub const fn new(scale: NonZeroU8, image: T) -> Self {
        Self { scale, image }
    }
}

impl<T: Render> Render for Scale<T> {
    fn render(&self, offset: Point, canvas: &mut Canvas) {
        let scale = i16::from(self.scale.get());
        let image = self.image.to_image();
        for (point, color) in image.pixels() {
            let point = point * scale + offset;
            for y in 0..scale {
                for x in 0..scale {
                    canvas.set_pixel(point.move_xy(x, y), color);
                }
            }
        }
    }
}
