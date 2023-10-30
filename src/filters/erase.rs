use crate::{Entity, Filter, Image};

#[derive(Debug, Clone, Copy)]
pub struct Erase;

impl Filter<Image> for Erase {
    fn filter(&self, _target: Image) -> Image {
        Image::default()
    }
}

impl Filter<Entity> for Erase {
    fn filter(&self, _target: Entity) -> Entity {
        Entity::default()
    }
}
