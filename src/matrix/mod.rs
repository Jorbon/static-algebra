mod from;
mod fmt;
// pub mod helper;
// mod core_ops;
// mod custom_ops;
// pub mod column_view;

use crate::{number::{Add1, Num0}, static_list::{RecursiveParts, StaticList, StaticListBase, StaticListRecursive, StaticListRecursiveMut, StaticListRecursiveOwned}};


#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Mat0<T, Column>
where
    Column: StaticList<T>,
{
    pd: core::marker::PhantomData<(T, Column)>,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Matrix<T, Column, Inner>
where
    Column: StaticList<T>,
    Inner: StaticList<Column>,
{
    pub(crate) inner: Inner,
    pub(crate) end: Column,
    pd: core::marker::PhantomData<T>,
}

impl<T, Column, Inner> Matrix<T, Column, Inner>
where
    Column: StaticList<T>,
    Inner: StaticList<Column>,
{
    #[inline]
    pub const fn push_column(inner: Inner, end: Column) -> Self {
        Self { inner, end, pd: core::marker::PhantomData }
    }
}


impl<T, Column> StaticList<Column> for Mat0<T, Column>
where
    Column: StaticList<T>,
{
    type Length = Num0;
}

impl<T, Column, Inner> StaticList<Column> for Matrix<T, Column, Inner>
where
    Column: StaticList<T>,
    Inner: StaticList<Column>,
{
    type Length = Add1<Inner::Length>;
}


impl<T, Column> StaticListBase<Column> for Mat0<T, Column>
where
    Column: StaticList<T>,
{}

impl<T, Column, Inner> StaticListRecursive<Column> for Matrix<T, Column, Inner>
where
    Column: StaticList<T>,
    Inner: StaticList<Column>,
{
    type Inner = Inner;
    
    #[inline]
    fn parts(&self) -> RecursiveParts<&Self::Inner, &Column> {
        RecursiveParts { inner: &self.inner, end: &self.end }
    }
}

impl<T, Column, Inner> StaticListRecursiveMut<Column> for Matrix<T, Column, Inner>
where
    Column: StaticList<T>,
    Inner: StaticList<Column>,
{
    
    #[inline]
    fn parts_mut(&mut self) -> RecursiveParts<&mut Self::Inner, &mut Column> {
        RecursiveParts { inner: &mut self.inner, end: &mut self.end }
    }
}

impl<T, Column, Inner> StaticListRecursiveOwned<Column> for Matrix<T, Column, Inner>
where
    Column: StaticList<T>,
    Inner: StaticList<Column>,
{
    type Inner = Inner;
    
    #[inline]
    fn parts_owned(self) -> RecursiveParts<Self::Inner, Column> {
        RecursiveParts { inner: self.inner, end: self.end }
    }
}


impl<T, Column: StaticList<T>> Mat0<T, Column> {
    pub const VALUE: Self = Self { pd: core::marker::PhantomData };
}

