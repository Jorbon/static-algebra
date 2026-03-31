pub mod from;
pub mod fmt;
// pub mod helper;
// pub mod core_ops;
// pub mod custom_ops;
// pub mod column_view;

// pub use column_view::*;


use crate::{Add1, Num0, RecursiveParts, StaticList, StaticListBase, StaticListRecursive, StaticListRecursiveMut, StaticListRecursiveOwned};


#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Mat0<T, Column: StaticList<T>>(
    core::marker::PhantomData<(T, Column)>,
);

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Matrix<T, Column: StaticList<T>, Inner: StaticList<Column>>(
    pub Inner,
    pub Column,
    core::marker::PhantomData<T>,
);


impl<T, Column: StaticList<T>> StaticList<Column> for Mat0<T, Column> {
    type Length = Num0;
}

impl<T, Column: StaticList<T>, Inner: StaticList<Column>> StaticList<Column> for Matrix<T, Column, Inner> {
    type Length = Add1<Inner::Length>;
}


impl<T, Column: StaticList<T>> StaticListBase<Column> for Mat0<T, Column> {}

impl<T, Column: StaticList<T>, Inner: StaticList<Column>> StaticListRecursive<Column> for Matrix<T, Column, Inner> {
    type Inner = Inner;
    
    #[inline]
    fn parts(&self) -> RecursiveParts<&Self::Inner, &Column> {
        RecursiveParts { inner: &self.0, end: &self.1 }
    }
}

impl<T, Column: StaticList<T>, Inner: StaticList<Column>> StaticListRecursiveMut<Column> for Matrix<T, Column, Inner> {
    
    #[inline]
    fn parts_mut(&mut self) -> RecursiveParts<&mut Self::Inner, &mut Column> {
        RecursiveParts { inner: &mut self.0, end: &mut self.1 }
    }
}

impl<T, Column: StaticList<T>, Inner: StaticList<Column>> StaticListRecursiveOwned<Column> for Matrix<T, Column, Inner> {
    type Inner = Inner;
    
    #[inline]
    fn parts_owned(self) -> RecursiveParts<Self::Inner, Column> {
        RecursiveParts { inner: self.0, end: self.1 }
    }
}


impl<T, Column: StaticList<T>> Mat0<T, Column> {
    pub const VALUE: Self = Self(core::marker::PhantomData);
}

impl<T, Column: StaticList<T>, Inner: StaticList<Column>> Matrix<T, Column, Inner> {
    #[inline]
    pub const fn with_inner(inner: Inner, end: Column) -> Self {
        Self(inner, end, core::marker::PhantomData)
    }
}

