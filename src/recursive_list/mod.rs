mod iterable_impl;
pub mod static_index_impl;
mod functor_impls;

use crate::{number::{Num0, NumAdd1, Number}, static_list::{StaticList, StaticListMut, StaticListOwned}};


/// More specific form of [`StaticList`] that has [`BaseCase`] and [`RecursiveCase`] variants.
/// Note that both cases must define [`RecursiveList::Inner`] and [`RecursiveList::contents`] because they must implement the same trait.
/// Using different traits for the two cases would require mutual exclusion in order to blanket impl other traits, which Rust doesn't support.
/// Mutual exclusion impls are instead achieved by associate splitting on the [`RecursiveList::Case`] field.
pub trait RecursiveList<T>:
    StaticList<T>
{
    type Case: RecursiveListCase<T, Self::Inner>;
    type Inner: RecursiveList<T>;
    fn base() -> impl RecursiveList<T, Case = BaseCase>;
    fn push(self, end: T) -> impl RecursiveList<T, Case = RecursiveCase, Inner = Self>;
    fn contents<'a>(&'a self) -> <Self::Case as RecursiveListCase<T, Self::Inner>>::Contents<'a> where T: 'a, Self::Inner: 'a;
}

pub trait RecursiveListMut<T>:
    RecursiveList<T> + StaticListMut<T>
{
    fn contents_mut<'a>(&'a self) -> <Self::Case as RecursiveListCase<T, Self::Inner>>::ContentsMut<'a> where T: 'a, Self::Inner: 'a;
}

pub trait RecursiveListOwned<T>:
    RecursiveList<T> + StaticListOwned<T>
{
    fn contents_owned(self) -> <Self::Case as RecursiveListCase<T, Self::Inner>>::ContentsOwned;
}


trait RecursiveListCaseSealed {}
#[allow(private_bounds)]
pub trait RecursiveListCase<T, Inner>:
    RecursiveListCaseSealed
where
    Inner: RecursiveList<T>,
{
    type Length: Number;
    type Contents<'a>: MaybeRecursiveContents<&'a T, &'a Inner> where T: 'a, Inner: 'a;
    type ContentsMut<'a>: MaybeRecursiveContents<&'a mut T, &'a mut Inner> where T: 'a, Inner: 'a;
    type ContentsOwned: MaybeRecursiveContents<T, Inner>;
}

pub struct BaseCase;
pub struct RecursiveCase;

impl RecursiveListCaseSealed for BaseCase {}
impl<T, Inner> RecursiveListCase<T, Inner> for BaseCase
where
    Inner: RecursiveList<T, Case = BaseCase>,
{
    type Length = Num0;
    type Contents<'a> = BaseContents where T: 'a, Inner: 'a;
    type ContentsMut<'a> = BaseContents where T: 'a, Inner: 'a;
    type ContentsOwned = BaseContents;
}

impl RecursiveListCaseSealed for RecursiveCase {}
impl<T, Inner> RecursiveListCase<T, Inner> for RecursiveCase
where
    Inner: RecursiveList<T>,
{
    type Length = NumAdd1<Inner::Length>;
    type Contents<'a> = RecursiveContents<&'a T, &'a Inner> where T: 'a, Inner: 'a;
    type ContentsMut<'a> = RecursiveContents<&'a mut T, &'a mut Inner> where T: 'a, Inner: 'a;
    type ContentsOwned = RecursiveContents<T, Inner>;
}


// None case for recursive list contents.
#[derive(Copy, Clone, Debug)]
pub struct BaseContents;

// Some case for recursive list contents.
#[derive(Copy, Clone, Debug)]
pub struct RecursiveContents<T, Inner> {
    pub inner: Inner,
    pub end: T,
}

trait MaybeRecursiveContents<T, Inner> {}
impl<T, Inner> MaybeRecursiveContents<T, Inner> for BaseContents {}
impl<T, Inner> MaybeRecursiveContents<T, Inner> for RecursiveContents<T, Inner> {}


impl<T, List> StaticList<T> for List
where
    List: RecursiveList<T>,
{
    type Length = <List::Case as RecursiveListCase<T, List::Inner>>::Length;
}

impl<T, List> StaticListMut<T> for List
where
    List: RecursiveListMut<T>,
{}

impl<T, List> StaticListOwned<T> for List
where
    List: RecursiveListOwned<T>,
{}

