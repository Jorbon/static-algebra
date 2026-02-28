pub mod properties;
pub mod helper;
pub mod core_ops;
pub mod custom_ops;

#[allow(unused_imports)]
pub use properties::*;
pub use helper::*;
#[allow(unused_imports)]
pub use core_ops::*;
pub use custom_ops::*;

use crate::{Add1, Num0, StaticList, StaticListBase, StaticListRecursive, StaticListRecursiveMut, StaticListRecursiveOwned};


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
    fn inner(&self) -> &Self::Inner {
        &self.0
    }
    
    #[inline]
    fn end(&self) -> &T {
        &self.1
    }
}

impl<T, Inner: StaticList<T>> StaticListRecursiveMut<T> for Vector<T, Inner> {
    
    #[inline]
    fn inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.0
    }
    
    #[inline]
    fn end_mut(&mut self) -> &mut T {
        &mut self.1
    }
}

impl<T, Inner: StaticList<T>> StaticListRecursiveOwned<T> for Vector<T, Inner> {
    
    #[inline]
    fn inner_owned(self) -> Self::Inner {
        self.0
    }
    
    #[inline]
    fn end_owned(self) -> T {
        self.1
    }
}


impl<T> Vec0<T> {
    pub const VALUE: Self = Self(core::marker::PhantomData);
}

