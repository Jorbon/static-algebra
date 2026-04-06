//! Blanket implementing runtime iterator and index methods for all recursive [`StaticList`] implementations.

use crate::{recursive_list::{BaseCase, RecursiveCase, RecursiveList, RecursiveListMut, RecursiveListOwned}, static_list::iterable::{Iterable, IterableMut, IterableOwned}};


/// Using to implement iterator behavior disjointly by associated types.
/// https://github.com/rust-lang/rfcs/pull/1672#issuecomment-1405377983
trait DisjointHelper<T, Case> {
    fn helper_iter<'a>(&'a self) -> impl Iterator<Item = &'a T> where T: 'a;
}

trait DisjointHelperMut<T, Case>:
    DisjointHelper<T, Case>
{
    fn helper_iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> where T: 'a;
}

trait DisjointHelperOwned<T, Case>:
    DisjointHelper<T, Case>
{
    fn helper_into_iter(self) -> impl Iterator<Item = T>;
}



impl<T, List> DisjointHelper<T, BaseCase> for List
where
    List: RecursiveList<T, Case = BaseCase>,
{
    #[inline]
    fn helper_iter<'a>(&'a self) -> impl Iterator<Item = &'a T> where T: 'a {
        core::iter::empty()
    }
}

impl<T, List> DisjointHelperMut<T, BaseCase> for List
where
    List: RecursiveListMut<T, Case = BaseCase>,
{
    #[inline]
    fn helper_iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> where T: 'a {
        core::iter::empty()
    }
}

impl<T, List> DisjointHelperOwned<T, BaseCase> for List
where
    List: RecursiveListOwned<T, Case = BaseCase>,
{
    #[inline]
    fn helper_into_iter(self) -> impl Iterator<Item = T> {
        core::iter::empty()
    }
}



impl<T, List> DisjointHelper<T, RecursiveCase> for List
where
    List: RecursiveList<T, Case = RecursiveCase>,
    List::Inner: Iterable<T>,
{
    #[inline]
    fn helper_iter<'a>(&'a self) -> impl Iterator<Item = &'a T> where T: 'a {
        let contents = self.contents();
        contents.inner.iter().chain(core::iter::once(contents.end))
    }
}

impl<T, List> DisjointHelperMut<T, RecursiveCase> for List
where
    List: RecursiveListMut<T, Case = RecursiveCase>,
    List::Inner: IterableMut<T> + DisjointHelperMut<T, <List::Inner as RecursiveList<T>>::Case>,
{
    #[inline]
    fn helper_iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> where T: 'a {
        let contents = self.contents_mut();
        contents.inner.iter_mut().chain(core::iter::once(contents.end))
    }
}

impl<T, List> DisjointHelperOwned<T, RecursiveCase> for List
where
    List: RecursiveListOwned<T, Case = RecursiveCase>,
    List::Inner: IterableOwned<T> + DisjointHelperOwned<T, <List::Inner as RecursiveList<T>>::Case>,
{
    #[inline]
    fn helper_into_iter(self) -> impl Iterator<Item = T> {
        let contents = self.contents_owned();
        contents.inner.into_iter().chain(core::iter::once(contents.end))
    }
}



impl<T, List> Iterable<T> for List
where
    List: RecursiveList<T> + DisjointHelper<T, List::Case>,
{
    #[inline]
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> where T: 'a {
        self.helper_iter()
    }
}

impl<T, List> IterableMut<T> for List
where
    List: RecursiveListMut<T> + DisjointHelperMut<T, List::Case>,
{
    #[inline]
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> where T: 'a {
        self.helper_iter_mut()
    }
}

impl<T, List> IterableOwned<T> for List
where
    List: RecursiveListOwned<T> + DisjointHelperOwned<T, List::Case>,
{
    #[inline]
    fn into_iter(self) -> impl Iterator<Item = T> {
        self.helper_into_iter()
    }
}

