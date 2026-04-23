//! Trait describing a static list of values, including static indexing and iteration

use crate::number::{NumAdd1, Number, StaticMinus};


/// Abstract trait describing a static list of values.
/// The sub-trait [`RecursiveList`] provides the actual trait bound implementations for use elsewhere in the crate.
pub trait StaticList<T> {
    type Length: Number;
}

pub trait Iterable<T> {
    fn iter(self) -> impl Iterator<Item = T>;
    
    #[inline]
    fn get(self, index: usize) -> Option<T>
    where
        Self: Sized,
    {
        self.iter().nth(index)
    }
}

pub trait StaticGet<T, Index>:
    StaticList<T, Length: StaticMinus<NumAdd1<Index>>>
where
    Index: Number,
{
    fn static_get(self) -> T;
}

pub trait StaticMap<T, U> {
    type MapOutput;
    
    fn map<F>(self, f: &F) -> Self::MapOutput
    where
        F: Fn(T) -> U;
}

pub trait StaticZip<T, U, R, ListR> {
    type ZipOutput;
    
    fn zip<F>(self, rhs: ListR, f: &F) -> Self::ZipOutput
    where
        F: Fn(T, R) -> U;
}

