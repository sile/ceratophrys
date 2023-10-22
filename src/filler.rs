use crate::{Canvas, Color, Point, Render};
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct Filler<T> {
    color: Color,
    image: T,
}

impl<T> Filler<T> {
    pub const fn new(color: Color, image: T) -> Self {
        Self { color, image }
    }
}

impl<T: Render> Render for Filler<T> {
    fn render(&self, offset: Point, canvas: &mut Canvas) {
        let image = self.image.to_image();

        let mut stack = image
            .size()
            .edge_points()
            .filter(|p| image.get_color(*p).unwrap_or_default().is_transparent())
            .collect::<Vec<_>>();
        let mut outer_points = BTreeSet::new();
        while let Some(point) = stack.pop() {
            if outer_points.contains(&point)
                || !image.size().contains(point)
                || !image.get_color(point).unwrap_or_default().is_transparent()
            {
                continue;
            }
            outer_points.insert(point);

            stack.push(point.move_x(-1));
            stack.push(point.move_x(1));
            stack.push(point.move_y(-1));
            stack.push(point.move_y(1));
        }

        image.render(offset, canvas);
        for (point, color) in image.pixels() {
            if color.is_transparent() && !outer_points.contains(&point) {
                canvas.set_pixel(point + offset, self.color);
            }
        }
    }
}
