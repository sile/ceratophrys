pub mod bmp;
pub mod gif;

mod canvas;
mod color;
mod image;
mod palette;
mod render;
mod size;
mod text_image;

pub use canvas::Canvas;
pub use color::Color;
pub use image::Image;
pub use palette::Palette;
pub use render::Render;
pub use size::Size;
pub use text_image::TextImage;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

impl Point {
    pub const ORIGIN: Self = Self::new(0, 0);

    pub const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    pub const fn move_x(self, x: i16) -> Self {
        Self::new(self.x.saturating_add(x), self.y)
    }

    pub const fn move_y(self, y: i16) -> Self {
        Self::new(self.x, self.y.saturating_add(y))
    }

    pub const fn move_xy(self, x: i16, y: i16) -> Self {
        Self::new(self.x.saturating_add(x), self.y.saturating_add(y))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, Self { x, y }: Self) -> Self::Output {
        Self::new(self.x.saturating_add(x), self.y.saturating_add(y))
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, Self { x, y }: Self) -> Self::Output {
        Self::new(self.x.saturating_sub(x), self.y.saturating_sub(y))
    }
}
