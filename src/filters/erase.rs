use crate::{filters::Filter, Image};

#[derive(Debug, Clone, Copy)]
pub struct Erase;

impl Filter for Erase {
    fn filter(&self, image: &mut Image) {
        *image = Image::default();
    }
}
