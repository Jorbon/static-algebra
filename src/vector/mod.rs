//! Algebraic vector type. Specialization of [`StaticList`] to include vector operations.

// pub mod view;
pub mod helper;
mod from;
// mod fmt;
pub mod ops;
mod core_ops;

use core::marker::PhantomData;

use crate::{recursive_list::{BaseCase, BaseContents, RecursiveCase, RecursiveContents, RecursiveList, RecursiveListCase}, static_list::{Iterable, Mappable}};


trait VectorSealed {}
/// Trait representing the actual concrete vector implementation types
/// Does not actually have any behavior described or required by it, but acts as a convenient way to label and refer to the included `RecursiveList` and `Iterable` bounds in downstream impl bounds
#[allow(private_bounds)]
pub trait Vector:
    VectorSealed + RecursiveList<Push = VecPush<Self>> + Iterable<Item = Self::T>
{}


#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Vec0<T>(PhantomData<T>);

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct VecPush<Inner>
where
    Inner: Vector,
{
    pub(crate) inner: Inner,
    pub(crate) end: Inner::T,
}


impl<T> RecursiveList for Vec0<T> {
    type T = T;
    type Case = BaseCase;
    type Base = Vec0<Self::T>;
    type Push = VecPush<Self>;
    type Inner = Vec0<Self::T>;
    
    const BASE: Self::Base = Vec0(PhantomData);
    
    fn push(self, end: Self::T) -> Self::Push {
        VecPush { inner: self, end }
    }
    
    fn contents(self) -> <Self::Case as RecursiveListCase<Self::T, Self::Inner>>::Contents {
        BaseContents
    }
}

impl<Inner> RecursiveList for VecPush<Inner>
where
    Inner: Vector,
{
    type T = Inner::T;
    type Case = RecursiveCase;
    type Base = Vec0<Self::T>;
    type Push = VecPush<Self>;
    type Inner = Inner;
    
    const BASE: Self::Base = Vec0(PhantomData);
    
    fn push(self, end: Self::T) -> Self::Push {
        VecPush { inner: self, end }
    }
    
    fn contents(self) -> <Self::Case as RecursiveListCase<Self::T, Self::Inner>>::Contents {
        RecursiveContents {
            inner: self.inner,
            end: self.end,
        }
    }
}


impl<T> VectorSealed for Vec0<T> {}
impl<T> Vector for Vec0<T> {}

impl<Inner> VectorSealed for VecPush<Inner> where Inner: Vector {}
impl<Inner> Vector for VecPush<Inner> where Inner: Vector {}


impl<T, R> Mappable<R> for Vec0<T> {
    type Mapped = Vec0<R>;
}

impl<Inner, R> Mappable<R> for VecPush<Inner>
where
    Inner: Vector + Mappable<R>,
    Inner::Mapped: Vector<T = R>,
{
    type Mapped = VecPush<Inner::Mapped>;
}



// impl<'a, List> Vector for &'a List
// where
//     List: Vector
// {
    
// }

