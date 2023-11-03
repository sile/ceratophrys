use crate::{Color, Filter, Image};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy)]
pub struct Fill(Color);

impl Fill {
    pub const fn new(color: Color) -> Self {
        Self(color)
    }
}

impl Filter for Fill {
    fn filter(&self, mut image: Image) -> Image {
        let mut stack = image
            .size()
            .edge_positions()
            .filter(|p| image.get_color(*p).unwrap_or_default().is_transparent())
            .collect::<Vec<_>>();
        let mut outer_positions = BTreeSet::new();
        while let Some(position) = stack.pop() {
            if outer_positions.contains(&position)
                || !image.size().contains(position)
                || !image
                    .get_color(position)
                    .unwrap_or_default()
                    .is_transparent()
            {
                continue;
            }
            outer_positions.insert(position);

            stack.push(position.move_x(-1));
            stack.push(position.move_x(1));
            stack.push(position.move_y(-1));
            stack.push(position.move_y(1));
        }

        for (position, color) in image.size().positions().zip(image.colors_mut().iter_mut()) {
            if color.is_transparent() && !outer_positions.contains(&position) {
                *color = self.0;
            }
        }

        for child in &mut image.children {
            *child = self.filter(std::mem::take(child));
        }

        image
    }
}
