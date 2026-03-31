// pub mod view;
pub mod helper;
mod from;
mod fmt;
mod ops;
mod core_ops;

// pub use view::*;
#[allow(unused_imports)]
pub use helper::*;

use crate::{Add1, Num0, RecursiveParts, StaticList, StaticListBase, StaticListRecursive, StaticListRecursiveMut, StaticListRecursiveOwned};


#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Vec0<T>(core::marker::PhantomData<T>);

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Vector<T, Inner: StaticList<T>>(pub Inner, pub T);


impl<T> StaticList<T> for Vec0<T> {
    type Length = Num0;
}

impl<T, Inner: StaticList<T>> StaticList<T> for Vector<T, Inner> {
    type Length = Add1<Inner::Length>;
}


impl<T> StaticListBase<T> for Vec0<T> {}

impl<T, Inner: StaticList<T>> StaticListRecursive<T> for Vector<T, Inner> {
    type Inner = Inner;
    
    #[inline]
    fn parts(&self) -> RecursiveParts<&Self::Inner, &T> {
        RecursiveParts { inner: &self.0, end: &self.1 }
    }
}

impl<T, Inner: StaticList<T>> StaticListRecursiveMut<T> for Vector<T, Inner> {
    
    #[inline]
    fn parts_mut(&mut self) -> RecursiveParts<&mut Self::Inner, &mut T> {
        RecursiveParts { inner: &mut self.0, end: &mut self.1 }
    }
}

impl<T, Inner: StaticList<T>> StaticListRecursiveOwned<T> for Vector<T, Inner> {
    type Inner = Inner;
    
    #[inline]
    fn parts_owned(self) -> RecursiveParts<Self::Inner, T> {
        RecursiveParts { inner: self.0, end: self.1 }
    }
}


impl<T> Vec0<T> {
    pub const VALUE: Self = Self(core::marker::PhantomData);
}

