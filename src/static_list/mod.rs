mod iterable_impl;
pub mod index;

pub use index::*;

use crate::{Add1, Number, Num0};


pub trait StaticList<T> {
    type Length: Number;
}

pub trait StaticListBase<T>: StaticList<T, Length = Num0> {}

#[derive(Copy, Clone, Debug)]
pub struct RecursiveParts<Inner, T> {
    pub inner: Inner,
    pub end: T,
}

pub trait StaticListRecursive<T>: StaticList<T, Length = Add1<<Self::Inner as StaticList<T>>::Length>> {
    type Inner: StaticList<T>;
    fn parts(&self) -> RecursiveParts<&Self::Inner, &T>;
}

pub trait StaticListRecursiveMut<T>: StaticListRecursive<T> {
    fn parts_mut(&mut self) -> RecursiveParts<&mut Self::Inner, &mut T>;
}

pub trait StaticListRecursiveOwned<T>: StaticList<T, Length = Add1<<Self::Inner as StaticList<T>>::Length>> {
    type Inner: StaticList<T>;
    fn parts_owned(self) -> RecursiveParts<Self::Inner, T>;
}

