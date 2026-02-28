use crate::{Add1, Iterable, IterableMut, IterableOwned, Num0, StaticList, StaticListBase, StaticListRecursive, StaticListRecursiveMut, StaticListRecursiveOwned};


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
        self.inner().iter()
    }
}

impl<T, Recursive> DisjointHelperMut<T, Add1<<<Self as StaticListRecursive<T>>::Inner as StaticList<T>>::Length>> for Recursive
where
    Self: StaticListRecursiveMut<T>,
    <Self as StaticListRecursive<T>>::Inner: StaticList<T> + IterableMut<T>,
{
    #[inline]
    fn helper_iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> where T: 'a {
        self.inner_mut().iter_mut()
    }
}

impl<T, Recursive> DisjointHelperOwned<T, Add1<<<Self as StaticListRecursive<T>>::Inner as StaticList<T>>::Length>> for Recursive
where
    Self: StaticListRecursiveOwned<T>,
    <Self as StaticListRecursive<T>>::Inner: StaticList<T> + IterableOwned<T>,
{
    #[inline]
    fn helper_into_iter(self) -> impl Iterator<Item = T> {
        self.inner_owned().into_iter()
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

