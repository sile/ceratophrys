pub mod bmp;
pub mod filters;
pub mod gif;
pub mod shapes;
// TODO: text

mod color;
mod entity;
mod filler;
mod filter;
mod image;
mod line;
mod offset;
mod point;
mod polygon;
mod silhouette;
mod size;

pub use color::Color;
pub use entity::Entity;
pub use filler::Filler;
pub use filter::Filter;
pub use image::Image;
pub use line::Line;
pub use offset::Offset;
pub use point::Point;
pub use polygon::Polygon;
pub use silhouette::Silhouette;
pub use size::Size;

pub type Position = Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pixel {
    pub position: Point,
    pub color: Color,
}

impl Pixel {
    pub const fn new(position: Point, color: Color) -> Self {
        Self { position, color }
    }

    pub fn map_position<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Point) -> Point,
    {
        self.position = f(self.position);
        self
    }
}
