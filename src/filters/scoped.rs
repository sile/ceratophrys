use crate::{Entity, Filter, Image};

#[derive(Debug, Clone)]
pub struct Scoped<F> {
    entity_name: String,
    filter: F,
}

impl<F> Scoped<F> {
    pub fn new(entity_name: &str, filter: F) -> Self {
        Self {
            entity_name: entity_name.to_owned(),
            filter,
        }
    }
}

impl<F: Filter<Image>> Filter<Image> for Scoped<F> {
    fn filter(&self, target: Image) -> Image {
        target
    }
}

impl<F: Filter<Entity>> Filter<Entity> for Scoped<F> {
    fn filter(&self, mut target: Entity) -> Entity {
        if target.name.as_ref() == Some(&self.entity_name) {
            target = self.filter.filter(target);
        } else {
            for child in &mut target.children {
                *child = self.filter(std::mem::take(child))
            }
        }
        target
    }
}
