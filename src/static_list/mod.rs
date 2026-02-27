pub mod iterable_impls;
pub mod index;

#[allow(unused_imports)]
pub use iterable_impls::*;
pub use index::*;


use crate::{Add1, Number, Number0};


pub trait StaticList<T> {
    type Length: Number;
}

pub trait StaticListBase<T>: StaticList<T, Length = Number0> {}

pub trait StaticListRecursive<T>: StaticList<T, Length = Add1<<Self::Inner as StaticList<T>>::Length>> {
    type Inner: StaticList<T>;
    fn inner(&self) -> &Self::Inner;
    fn end(&self) -> &T;
}

pub trait StaticListRecursiveMut<T>: StaticListRecursive<T> {
    fn inner_mut(&mut self) -> &mut Self::Inner;
    fn end_mut(&mut self) -> &mut T;
}

pub trait StaticListRecursiveOwned<T>: StaticListRecursive<T> {
    fn inner_owned(self) -> Self::Inner;
    fn end_owned(self) -> T;
}

