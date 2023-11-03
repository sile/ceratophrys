use crate::Image;

pub trait Filter {
    fn filter(&self, image: Image) -> Image;
}

impl Filter for Box<dyn Filter> {
    fn filter(&self, image: Image) -> Image {
        (**self).filter(image)
    }
}

impl<F> Filter for &[F]
where
    F: Filter,
{
    fn filter(&self, image: Image) -> Image {
        self.iter().fold(image, |acc, x| x.filter(acc))
    }
}

impl<F> Filter for Vec<F>
where
    F: Filter,
{
    fn filter(&self, image: Image) -> Image {
        self.as_slice().filter(image)
    }
}
