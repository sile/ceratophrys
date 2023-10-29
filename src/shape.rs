use crate::{Color, Image};

pub trait Shape {
    fn to_image(&self, color: Color) -> Image;
}
