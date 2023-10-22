use crate::{Canvas, Image, Point};

pub trait Render {
    fn render(&self, offset: Point, canvas: &mut Canvas);

    fn to_image(&self) -> Image {
        let mut canvas = Canvas::new();
        self.render(Point::ORIGIN, &mut canvas);
        canvas.to_image()
    }
}

impl Render for &dyn Render {
    fn render(&self, point: Point, canvas: &mut Canvas) {
        (*self).render(point, canvas);
    }
}

impl Render for Box<dyn Render> {
    fn render(&self, point: Point, canvas: &mut Canvas) {
        (**self).render(point, canvas);
    }
}

impl<T: Render> Render for &T {
    fn render(&self, point: Point, canvas: &mut Canvas) {
        (*self).render(point, canvas);
    }
}

impl<T: Render> Render for Box<T> {
    fn render(&self, point: Point, canvas: &mut Canvas) {
        (**self).render(point, canvas);
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
