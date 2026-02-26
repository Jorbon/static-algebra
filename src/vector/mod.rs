pub mod properties;
pub mod helper;
pub mod std_ops;
pub mod custom_ops;

#[allow(unused_imports)] pub use properties::*;
#[allow(unused_imports)] pub use helper::*;
#[allow(unused_imports)] pub use std_ops::*;
#[allow(unused_imports)] pub use custom_ops::*;

use std::marker::PhantomData;

use crate::{StaticIndex, StaticIndexFromEnd, StaticList, StaticMinus, Count, Count0, CountTrait};


#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Vec0<T>(PhantomData<T>);
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Vector<T, Inner: StaticList<T>>(pub Inner, pub T);

impl<T> StaticList<T> for Vec0<T> {
    type Count = Count0;
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> where T: 'a {
        std::iter::empty()
    }
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> where T: 'a {
        std::iter::empty()
    }
    fn iter_owned(self) -> impl Iterator<Item = T> {
        std::iter::empty()
    }
}
impl<T, Inner: StaticList<T>> StaticList<T> for Vector<T, Inner> {
    type Count = Count<Inner::Count>;
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> where T: 'a {
        self.0.iter().chain(std::iter::once(&self.1))
    }
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> where T: 'a {
        self.0.iter_mut().chain(std::iter::once(&mut self.1))
    }
    fn iter_owned(self) -> impl Iterator<Item = T> {
        self.0.iter_owned().chain(std::iter::once(self.1))
    }
}


impl<T, Inner: StaticList<T>> StaticIndexFromEnd<T, Count0> for Vector<T, Inner> {
    fn static_index_from_end(&self) -> &T {
        &self.1
    }
    fn static_index_from_end_mut(&mut self) -> &mut T {
        &mut self.1
    }
    fn static_index_from_end_owned(self) -> T {
        self.1
    }
}
impl<T, VectorInner, CountInner> StaticIndexFromEnd<T, Count<CountInner>> for Vector<T, VectorInner>
where
    VectorInner: StaticList<T> + StaticIndexFromEnd<T, CountInner>,
    CountInner: CountTrait,
{
    fn static_index_from_end(&self) -> &T {
        VectorInner::static_index_from_end(&self.0)
    }
    fn static_index_from_end_mut(&mut self) -> &mut T {
        VectorInner::static_index_from_end_mut(&mut self.0)
    }
    fn static_index_from_end_owned(self) -> T {
        VectorInner::static_index_from_end_owned(self.0)
    }
}

impl<T, Inner, C> StaticIndex<T, C> for Vector<T, Inner>
where
    Inner: StaticList<T>,
    C: CountTrait,
    Inner::Count: StaticMinus<C>,
    Self: StaticList<T, Count = Count<Inner::Count>> + StaticIndexFromEnd<T, <Inner::Count as StaticMinus<C>>::Difference>,
{
    type LengthMinusOne = Inner::Count;
}

#[inline]
pub const fn vec0<T>() -> Vec0<T> {
    Vec0(PhantomData)
}


