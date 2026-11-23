//! Trait describing a static list of values, including static indexing and iteration

use crate::number::Number;


/// Abstract trait describing a static list of values.
/// The sub-trait [`RecursiveList`] provides the actual trait bound implementations for use elsewhere in the crate.
pub trait StaticList {
    type Item;
    type Length: Number;
}

pub trait Iterable {
    type Item;
    
    fn iter(self) -> impl Iterator<Item = Self::Item>;
    
    #[inline]
    fn get(self, index: usize) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.iter().nth(index)
    }
}

pub trait StaticGet<T, Index>
where
    Index: Number,
{
    fn static_get(self) -> T;
}

pub trait Mappable<R> {
    type Mapped;
}

pub trait StaticMap<T, R>: Mappable<R> {
    fn map<F>(self, f: &F) -> Self::Mapped
    where
        F: Fn(T) -> R;
}

pub trait StaticZip<Rhs, T, U, R>: Mappable<R> {
    fn zip<F>(self, rhs: Rhs, f: &F) -> Self::Mapped
    where
        F: Fn(T, U) -> R;
}

