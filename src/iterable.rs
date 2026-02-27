
pub trait Iterable<T> {
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> where T: 'a;
}

pub trait IterableMut<T> {
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> where T: 'a;
}

pub trait IterableOwned<T> {
    fn into_iter(self) -> impl Iterator<Item = T>;
}
