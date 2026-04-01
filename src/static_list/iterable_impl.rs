//! Blanket implementing runtime iterator and index methods for all recursive [`StaticList`] implementations.

use crate::{iterable::{Iterable, IterableMut, IterableOwned}, number::{Add1, Num0}, static_list::{StaticList, StaticListBase, StaticListRecursive, StaticListRecursiveMut, StaticListRecursiveOwned}};


/// Using to implement iterator behavior disjointly by associated types.
/// https://github.com/rust-lang/rfcs/pull/1672#issuecomment-1405377983
trait DisjointHelper<T, Type> {
    fn helper_iter<'a>(&'a self) -> impl Iterator<Item = &'a T> where T: 'a;
}

trait DisjointHelperMut<T, Type> {
    fn helper_iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> where T: 'a;
}

trait DisjointHelperOwned<T, Type> {
    fn helper_into_iter(self) -> impl Iterator<Item = T>;
}



impl<T, Base> DisjointHelper<T, Num0> for Base
where Self: StaticListBase<T>
{
    #[inline]
    fn helper_iter<'a>(&'a self) -> impl Iterator<Item = &'a T> where T: 'a {
        core::iter::empty()
    }
}

impl<T, Base> DisjointHelperMut<T, Num0> for Base
where Self: StaticListBase<T>
{
    #[inline]
    fn helper_iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> where T: 'a {
        core::iter::empty()
    }
}

impl<T, Base> DisjointHelperOwned<T, Num0> for Base
where Self: StaticListBase<T>
{
    #[inline]
    fn helper_into_iter(self) -> impl Iterator<Item = T> {
        core::iter::empty()
    }
}



impl<T, Recursive> DisjointHelper<T, Add1<<<Self as StaticListRecursive<T>>::Inner as StaticList<T>>::Length>> for Recursive
where
    Self: StaticListRecursive<T>,
    <Self as StaticListRecursive<T>>::Inner: StaticList<T> + Iterable<T>,
{
    #[inline]
    fn helper_iter<'a>(&'a self) -> impl Iterator<Item = &'a T> where T: 'a {
        let parts = self.parts();
        parts.inner.iter().chain(core::iter::once(parts.end))
    }
}

impl<T, Recursive> DisjointHelperMut<T, Add1<<<Self as StaticListRecursive<T>>::Inner as StaticList<T>>::Length>> for Recursive
where
    Self: StaticListRecursiveMut<T>,
    <Self as StaticListRecursive<T>>::Inner: StaticList<T> + IterableMut<T>,
{
    #[inline]
    fn helper_iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> where T: 'a {
        let parts = self.parts_mut();
        parts.inner.iter_mut().chain(core::iter::once(parts.end))
    }
}

impl<T, Recursive> DisjointHelperOwned<T, Add1<<<Self as StaticListRecursiveOwned<T>>::Inner as StaticList<T>>::Length>> for Recursive
where
    Self: StaticListRecursiveOwned<T>,
    <Self as StaticListRecursiveOwned<T>>::Inner: StaticList<T> + IterableOwned<T>,
{
    #[inline]
    fn helper_into_iter(self) -> impl Iterator<Item = T> {
        let parts = self.parts_owned();
        parts.inner.into_iter().chain(core::iter::once(parts.end))
    }
}



impl<T, List> Iterable<T> for List
where List: StaticList<T> + DisjointHelper<T, List::Length>
{
    #[inline]
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> where T: 'a {
        self.helper_iter()
    }
}

impl<T, List> IterableMut<T> for List
where List: StaticList<T> + DisjointHelperMut<T, List::Length>
{
    #[inline]
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> where T: 'a {
        self.helper_iter_mut()
    }
}

impl<T, List> IterableOwned<T> for List
where List: StaticList<T> + DisjointHelperOwned<T, List::Length>
{
    #[inline]
    fn into_iter(self) -> impl Iterator<Item = T> {
        self.helper_into_iter()
    }
}

