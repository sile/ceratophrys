use crate::{Filter, Image, Pixel, Position};

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

    pub fn image_pixels(self, pixels: impl IntoIterator<Item = Pixel>) -> Self {
        //self.color(Color::ch(' '), Color::BLACK);
        Self {
            image: Image::from_iter(pixels),
            ..self
        }
    }

    // TODO: flatten

    pub fn offset(self, offset: Position) -> Self {
        Self { offset, ..self }
    }

    pub fn name(self, name: &str) -> Self {
        Self {
            name: Some(name.to_owned()),
            ..self
        }
    }

    pub fn child(mut self, child: Entity) -> Self {
        self.children.push(child);
        self
    }

    pub fn pixels(&self) -> impl '_ + Iterator<Item = Pixel> {
        let pixels = self.image.pixels();
        let children_pixels = self.children.iter().flat_map(|child| child.pixels());
        pixels
            .chain(Box::new(children_pixels) as Box<dyn Iterator<Item = Pixel>>)
            .map(|pixel| pixel.map_position(|p| p + self.offset))
    }

    pub fn to_image(&self) -> Image {
        Image::from_iter(self.pixels())
    }

    pub fn filter<F: Filter<Entity>>(self, filter: F) -> Self {
        filter.filter(self)
    }
}
