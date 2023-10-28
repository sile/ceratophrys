use crate::{Canvas, Image, Point, Position, Render};

#[derive(Debug, Default, Clone)]
pub struct Entity {
    pub image: Image,
    pub offset: Position,
    pub name: Option<String>,
    pub children: Vec<Entity>,
}

impl Entity {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn image(self, image: impl Into<Image>) -> Self {
        Self {
            image: image.into(),
            ..self
        }
    }

    pub fn offset(self, offset: Position) -> Self {
        Self { offset, ..self }
    }

    pub fn name(self, name: &str) -> Self {
        Self {
            name: Some(name.to_owned()),
            ..self
        }
    }

    pub fn child(mut self, child: impl Into<Entity>) -> Self {
        self.children.push(child.into());
        self
    }
}

impl Render for Entity {
    fn render(&self, offset: Point, canvas: &mut Canvas) {
        let offset = offset + self.offset;
        self.image.render(offset, canvas);
        for child in &self.children {
            child.render(offset, canvas);
        }
    }
}
