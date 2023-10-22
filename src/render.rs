use crate::{Canvas, Image, Point};

pub trait Render {
    fn render(&self, point: Point, canvas: &mut Canvas);

    fn to_image(&self) -> Image {
        let mut canvas = Canvas::new();
        self.render(Point::ORIGIN, &mut canvas);
        canvas.to_image()
    }
}

impl<T: Render> Render for Vec<T> {
    fn render(&self, point: Point, canvas: &mut Canvas) {
        for item in self {
            item.render(point, canvas);
        }
    }
}

impl<T: Render, const N: usize> Render for [T; N] {
    fn render(&self, point: Point, canvas: &mut Canvas) {
        for item in self {
            item.render(point, canvas);
        }
    }
}
