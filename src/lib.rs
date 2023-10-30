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
mod point;
mod silhouette;
mod size;

pub use animation::Animation;
pub use color::Color;
pub use entity::Entity;
pub use filter::Filter;
pub use image::Image;
pub use pixel::Pixel;
pub use point::Point;
pub use silhouette::Silhouette;
pub use size::Size;

pub type Position = Point;
