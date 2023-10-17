use crate::{Canvas, Image, Point};

pub trait Render {
    fn render(&self, point: Point, canvas: &mut Canvas);

    fn to_image(&self) -> Image {
        let mut canvas = Canvas::new();
        self.render(Point::ORIGIN, &mut canvas);
        canvas.to_image()
    }
}
