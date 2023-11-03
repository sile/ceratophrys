use crate::{Filter, Image};

#[derive(Debug, Clone, Copy)]
pub struct Erase;

impl Filter for Erase {
    fn filter(&self, _target: Image) -> Image {
        Image::default()
    }
}
