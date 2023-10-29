use crate::Entity;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Frame {
    pub entity: Entity,
    pub duration: Duration,
}

impl Frame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn entity(mut self, entity: Entity) -> Self {
        self.entity = entity;
        self
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            duration: Duration::from_secs(1),
        }
    }
}
