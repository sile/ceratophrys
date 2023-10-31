pub mod bmp;
pub mod filters;
pub mod gif;
pub mod shapes;

mod animation;
mod color;
mod entity;
mod filter;
mod image;
mod pixel;
mod position;
mod size;

pub use animation::Animation;
pub use color::Color;
pub use entity::Entity;
pub use filter::Filter;
pub use image::Image;
pub use pixel::Pixel;
pub use position::Position;
pub use size::Size;

pub const fn xy(x: i16, y: i16) -> Position {
    Position::xy(x, y)
}

pub const fn square(side: u16) -> Size {
    Size::square(side)
}
