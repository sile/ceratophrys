use crate::Image;

pub trait Filter {
    fn filter(&self, image: &mut Image);
}

impl Filter for Box<dyn Filter> {
    fn filter(&self, image: &mut Image) {
        (**self).filter(image);
    }
}

impl<F> Filter for &[F]
where
    F: Filter,
{
    fn filter(&self, image: &mut Image) {
        for f in self.iter() {
            f.filter(image);
        }
    }
}

impl<F> Filter for Vec<F>
where
    F: Filter,
{
    fn filter(&self, image: &mut Image) {
        self.as_slice().filter(image)
    }
}
