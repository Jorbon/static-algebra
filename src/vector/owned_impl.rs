use core::marker::PhantomData;

use crate::{recursive_list::{BaseCase, BaseContents, RecursiveCase, RecursiveContents, RecursiveList, RecursiveListCase}, static_list::Mappable, vector::{Vec0, VecPush, Vector, VectorSealed}};


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

