use crate::Point;

#[derive(Debug, Clone, Copy)]
pub struct Offset<T> {
    pub offset: Point,
    pub image: T,
}

impl<T> Offset<T> {
    pub const fn new(offset: Point, image: T) -> Self {
        Self { offset, image }
    }
}

// impl<T: Render> Render for Offset<T> {
//     fn render(&self, point: Point, canvas: &mut crate::Canvas) {
//         self.image.render(point + self.offset, canvas);
//     }
// }
