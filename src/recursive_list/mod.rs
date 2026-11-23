mod iterable_impl;
mod static_index_impl;
mod static_map_impl;

use crate::{number::{Num0, NumAdd1, Number}, static_list::StaticList};


/// More specific form of [`StaticList`] that has [`BaseCase`] and [`RecursiveCase`] variants.
/// Note that both cases must define [`RecursiveList::Inner`] and [`RecursiveList::contents`] because they must implement the same trait.
/// Using different traits for the two cases would require mutual exclusion in order to blanket impl other traits, which Rust doesn't support.
/// Mutual exclusion impls are instead achieved by associate splitting on the [`RecursiveList::Case`] field.
pub trait RecursiveList:
    Sized + StaticList
{
    type T;
    type Case: RecursiveListCase<Self::T, Self::Inner>;
    type Inner: RecursiveList<
        T = Self::T,
    >;
    type Base: RecursiveList<
        T = Self::T,
        Case = BaseCase,
        Length = Num0,
        Inner = Self::Base,
        Base = Self::Base,
    >;
    type Push: RecursiveList<
        T = Self::T,
        Case = RecursiveCase,
        Length = NumAdd1<Self::Length>,
        Inner = Self,
        Base = Self::Base,
    >;
    
    const BASE: Self::Base;
    
    fn push(self, end: Self::T) -> Self::Push;
    
    fn contents(self) -> <Self::Case as RecursiveListCase<Self::T, Self::Inner>>::Contents;
}


trait RecursiveListCaseSealed {}
#[allow(private_bounds)]
pub trait RecursiveListCase<T, Inner>:
    RecursiveListCaseSealed
where
    Inner: RecursiveList<T = T>,
{
    type Length: Number;
    type Contents: MaybeRecursiveContents<T, Inner>;
}

pub struct BaseCase;
pub struct RecursiveCase;

impl RecursiveListCaseSealed for BaseCase {}
impl<T, Inner> RecursiveListCase<T, Inner> for BaseCase
where
    Inner: RecursiveList<T = T, Case = BaseCase>,
{
    type Length = Num0;
    type Contents = BaseContents;
}

impl RecursiveListCaseSealed for RecursiveCase {}
impl<T, Inner> RecursiveListCase<T, Inner> for RecursiveCase
where
    Inner: RecursiveList<T = T>,
{
    type Length = NumAdd1<Inner::Length>;
    type Contents = RecursiveContents<T, Inner>;
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



impl<List> StaticList for List
where
    List: RecursiveList,
{
    type Item = List::T;
    type Length = <List::Case as RecursiveListCase<List::T, List::Inner>>::Length;
}

