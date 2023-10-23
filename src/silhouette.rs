use crate::{Canvas, Point, Render};

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
        todo!()
    }
}
