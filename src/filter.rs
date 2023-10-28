pub trait Filter<T> {
    fn filter(&self, target: T) -> T;
}
