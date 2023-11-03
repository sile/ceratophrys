pub mod bmp; // TODO: feature
pub mod filters;
pub mod gif; // TODO: feature
pub mod shapes;

mod animation;
mod color;
mod filter;
mod image;
mod pixel;
mod position;
mod size;

pub use animation::Animation;
pub use color::Color;
pub use filter::Filter; // TODO: move to filters
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
