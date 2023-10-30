pub trait Filter<T> {
    fn filter(&self, target: T) -> T;
}

impl<T> Filter<T> for Box<dyn Filter<T>> {
    fn filter(&self, target: T) -> T {
        (**self).filter(target)
    }
}

impl<T, F> Filter<T> for &[F]
where
    F: Filter<T>,
{
    fn filter(&self, target: T) -> T {
        self.iter().fold(target, |acc, x| x.filter(acc))
    }
}

impl<T, F> Filter<T> for Vec<F>
where
    F: Filter<T>,
{
    fn filter(&self, target: T) -> T {
        self.as_slice().filter(target)
    }
}
