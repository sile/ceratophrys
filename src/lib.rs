pub mod bmp;
pub mod filters;
pub mod gif;
pub mod shapes;
// TODO: text

mod canvas;
mod color;
mod entity;
mod filler;
mod filter;
mod image;
mod line;
mod offset;
mod palette;
mod point;
mod polygon;
mod rectangle;
mod silhouette;
mod size;
mod text_image;

pub use canvas::Canvas;
pub use color::Color;
pub use entity::Entity;
pub use filler::Filler;
pub use filter::Filter;
pub use image::Image;
pub use line::Line;
pub use offset::Offset;
pub use palette::Palette;
pub use point::Point;
pub use polygon::Polygon;
pub use rectangle::Rectangle;
pub use silhouette::Silhouette;
pub use size::Size;
pub use text_image::TextImage;

pub type Position = Point;
