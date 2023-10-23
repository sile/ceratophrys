use crate::{Canvas, Color, Point, Render};

#[derive(Debug, Clone, Copy)]
pub struct Silhouette<T> {
    image: T,
}

impl<T> Silhouette<T> {
    pub const fn new(image: T) -> Self {
        Self { image }
    }
}

impl<T: Render> Render for Silhouette<T> {
    fn render(&self, offset: Point, canvas: &mut Canvas) {
        let image = self.image.to_image();
        for (point, _color) in image.pixels() {
            canvas.set_pixel(point + offset, Color::rgb_hex(0x000000));
        }
    }
}
