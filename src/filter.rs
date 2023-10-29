use crate::Animation;

pub trait Filter<T> {
    fn filter(&self, target: T) -> T;
}

impl<T, F> Filter<Animation<F>> for T
where
    T: Filter<F>,
{
    fn filter(&self, mut target: Animation<F>) -> Animation<F> {
        target.frames = target
            .frames
            .into_iter()
            .map(|frame| self.filter(frame))
            .collect();
        target
    }
}
