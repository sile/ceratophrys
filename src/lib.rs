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
mod region;
mod size;

pub use animation::Animation;
pub use color::Color;
pub use image::Image;
pub use pixel::Pixel;
pub use position::Position;
pub use region::Region;
pub use size::Size;

pub const fn xy(x: i16, y: i16) -> Position {
    Position::xy(x, y)
}

pub const fn yx(y: i16, x: i16) -> Position {
    Position::xy(x, y)
}

pub const fn x(x: i16) -> Position {
    xy(x, 0)
}

pub const fn y(y: i16) -> Position {
    xy(0, y)
}

pub const fn wh(width: u16, height: u16) -> Size {
    Size::new(width, height)
}

pub const fn square(side: u16) -> Size {
    Size::square(side)
}
