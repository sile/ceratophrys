use crate::{filters::Filter, Image};

#[derive(Debug, Clone)]
pub struct Scoped<F> {
    target_name: String,
    filter: F,
}

impl<F> Scoped<F> {
    pub fn new(target_name: &str, filter: F) -> Self {
        Self {
            target_name: target_name.to_owned(),
            filter,
        }
    }
}

impl<F: Filter> Filter for Scoped<F> {
    fn filter(&self, image: &mut Image) {
        if image.name.as_ref() == Some(&self.target_name) {
            self.filter.filter(image);
        } else {
            for child in &mut image.children {
                self.filter(child);
            }
        }
    }
}
