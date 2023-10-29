use crate::{Color, Entity, Filter, Image};
use std::collections::BTreeSet;

pub fn fill<T>(color: Color, target: T) -> T
where
    Fill: Filter<T>,
{
    Fill::new(color).filter(target)
}

#[derive(Debug, Clone, Copy)]
pub struct Fill(Color);

impl Fill {
    pub const fn new(color: Color) -> Self {
        Self(color)
    }
}

impl Filter<Image> for Fill {
    fn filter(&self, mut image: Image) -> Image {
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

        for (position, color) in image.size().points().zip(image.colors_mut().iter_mut()) {
            if color.is_transparent() && !outer_points.contains(&position) {
                *color = self.0;
            }
        }
        image
    }
}

impl Filter<Entity> for Fill {
    fn filter(&self, mut target: Entity) -> Entity {
        target.image = self.filter(target.image);
        for child in &mut target.children {
            *child = self.filter(std::mem::take(child));
        }
        target
    }
}
