//! Algebraic matrix type. Specialization of [`StaticList<StaticList<T>>`] for matrix operations.

// pub mod helper;
mod from;
mod fmt;
// mod ops;
// mod core_ops;
// pub mod column_view;

use crate::{number::{NumAdd1, Num0}, static_list::{RecursiveContents, StaticList, StaticListBaseCase, StaticListRecursiveCase, StaticListRecursiveCaseMut, StaticListRecursiveCaseOwned}};


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


impl<T, Column> StaticListSealed for Mat0<T, Column> {}
impl<T, Column, Inner> StaticListSealed

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
    type Length = NumAdd1<Inner::Length>;
}


impl<T, Column> StaticListBaseCase<Column> for Mat0<T, Column>
where
    Column: StaticList<T>,
{}

impl<T, Column, Inner> StaticListRecursiveCase<Column> for Matrix<T, Column, Inner>
where
    Column: StaticList<T>,
    Inner: StaticList<Column>,
{
    type Inner = Inner;
    
    #[inline]
    fn parts(&self) -> RecursiveContents<&Self::Inner, &Column> {
        RecursiveContents { inner: &self.inner, end: &self.end }
    }
}

impl<T, Column, Inner> StaticListRecursiveCaseMut<Column> for Matrix<T, Column, Inner>
where
    Column: StaticList<T>,
    Inner: StaticList<Column>,
{
    
    #[inline]
    fn parts_mut(&mut self) -> RecursiveContents<&mut Self::Inner, &mut Column> {
        RecursiveContents { inner: &mut self.inner, end: &mut self.end }
    }
}

impl<T, Column, Inner> StaticListRecursiveCaseOwned<Column> for Matrix<T, Column, Inner>
where
    Column: StaticList<T>,
    Inner: StaticList<Column>,
{
    type Inner = Inner;
    
    #[inline]
    fn parts_owned(self) -> RecursiveContents<Self::Inner, Column> {
        RecursiveContents { inner: self.inner, end: self.end }
    }
}


impl<T, Column: StaticList<T>> Mat0<T, Column> {
    pub const VALUE: Self = Self { pd: core::marker::PhantomData };
}

