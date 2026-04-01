// pub mod view;
pub mod helper;
mod from;
mod fmt;
mod ops;
mod core_ops;

use crate::{number::{Add1, Num0}, static_list::{RecursiveParts, StaticList, StaticListBase, StaticListRecursive, StaticListRecursiveMut, StaticListRecursiveOwned}};


#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Vec0<T> {
    pd: core::marker::PhantomData<T>
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Vector<T, Inner>
where
    Inner: StaticList<T>,
{
    pub(crate) inner: Inner,
    pub(crate) end: T,
}

impl<T, Inner> Vector<T, Inner>
where
    Inner: StaticList<T>,
{
    #[inline]
    pub const fn push(inner: Inner, end: T) -> Self {
        Self { inner, end }
    }
}


impl<T> StaticList<T> for Vec0<T> {
    type Length = Num0;
}

impl<T, Inner> StaticList<T> for Vector<T, Inner>
where
    Inner: StaticList<T>,
{
    type Length = Add1<Inner::Length>;
}


impl<T> StaticListBase<T> for Vec0<T> {}

impl<T, Inner> StaticListRecursive<T> for Vector<T, Inner>
where
    Inner: StaticList<T>,
{
    type Inner = Inner;
    
    #[inline]
    fn parts(&self) -> RecursiveParts<&Self::Inner, &T> {
        RecursiveParts { inner: &self.inner, end: &self.end }
    }
}

impl<T, Inner> StaticListRecursiveMut<T> for Vector<T, Inner>
where
    Inner: StaticList<T>,
{
    #[inline]
    fn parts_mut(&mut self) -> RecursiveParts<&mut Self::Inner, &mut T> {
        RecursiveParts { inner: &mut self.inner, end: &mut self.end }
    }
}

impl<T, Inner> StaticListRecursiveOwned<T> for Vector<T, Inner>
where
    Inner: StaticList<T>,
{
    type Inner = Inner;
    
    #[inline]
    fn parts_owned(self) -> RecursiveParts<Self::Inner, T> {
        RecursiveParts { inner: self.inner, end: self.end }
    }
}


impl<T> Vec0<T> {
    pub const VALUE: Self = Self { pd: core::marker::PhantomData };
}

