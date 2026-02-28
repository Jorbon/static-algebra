pub mod properties;
// pub mod helper;
// pub mod core_ops;
// pub mod custom_ops;
pub mod column_view;

pub use column_view::*;


use crate::{Add1, Num0, StaticList, StaticListBase, StaticListRecursive, StaticListRecursiveMut, StaticListRecursiveOwned};


#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Mat0<T, Row: StaticList<T>>(
    core::marker::PhantomData<(T, Row)>,
);

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Matrix<T, Row: StaticList<T>, Inner: StaticList<Row>>(
    pub Inner,
    pub Row,
    core::marker::PhantomData<T>,
);


impl<T, Row: StaticList<T>> StaticList<Row> for Mat0<T, Row> {
    type Length = Num0;
}

impl<T, Row: StaticList<T>, Inner: StaticList<Row>> StaticList<Row> for Matrix<T, Row, Inner> {
    type Length = Add1<Inner::Length>;
}


impl<T, Row: StaticList<T>> StaticListBase<Row> for Mat0<T, Row> {}

impl<T, Row: StaticList<T>, Inner: StaticList<Row>> StaticListRecursive<Row> for Matrix<T, Row, Inner> {
    type Inner = Inner;
    
    #[inline]
    fn inner(&self) -> &Self::Inner {
        &self.0
    }
    
    #[inline]
    fn end(&self) -> &Row {
        &self.1
    }
}

impl<T, Row: StaticList<T>, Inner: StaticList<Row>> StaticListRecursiveMut<Row> for Matrix<T, Row, Inner> {
    
    #[inline]
    fn inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.0
    }
    
    #[inline]
    fn end_mut(&mut self) -> &mut Row {
        &mut self.1
    }
}

impl<T, Row: StaticList<T>, Inner: StaticList<Row>> StaticListRecursiveOwned<Row> for Matrix<T, Row, Inner> {
    type Inner = Inner;
    
    #[inline]
    fn inner_owned(self) -> Self::Inner {
        self.0
    }
    
    #[inline]
    fn end_owned(self) -> Row {
        self.1
    }
}


impl<T, Row: StaticList<T>> Mat0<T, Row> {
    pub const VALUE: Self = Self(core::marker::PhantomData);
}

impl<T, Row: StaticList<T>, Inner: StaticList<Row>> Matrix<T, Row, Inner> {
    #[inline]
    pub const fn with_inner(inner: Inner, end: Row) -> Self {
        Self(inner, end, core::marker::PhantomData)
    }
}

