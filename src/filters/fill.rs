use crate::{filters::Filter, Color, Image, Position, Size};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy)]
pub struct Fill(Color);

impl Fill {
    pub const fn new(color: Color) -> Self {
        Self(color)
    }
}

impl Filter for Fill {
    fn filter(&self, image: &mut Image) {
        let start = image.positions().fold(Position::MAX, Position::min);
        let size = image.positions().collect::<Size>();
        let mut stack = size.edge_positions().map(|p| p + start).collect::<Vec<_>>();
        let mut outer_positions = BTreeSet::new();
        while let Some(position) = stack.pop() {
            if outer_positions.contains(&position)
                || !size.contains(position - start)
                || !image.get_color(position).is_transparent()
            {
                continue;
            }
            outer_positions.insert(position);

            stack.push(position.move_x(-1));
            stack.push(position.move_x(1));
            stack.push(position.move_y(-1));
            stack.push(position.move_y(1));
        }

        for position in size.positions().map(|p| p + start) {
            if !outer_positions.contains(&position) && image.get_color(position).is_transparent() {
                image.pixels.insert(position, self.0);
            }
        }
    }
}
