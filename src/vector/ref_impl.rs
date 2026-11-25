use core::marker::PhantomData;

use crate::{recursive_list::{BaseCase, BaseContents, RecursiveCase, RecursiveContents, RecursiveList, RecursiveListCase}, static_list::Mappable, vector::{Vec0, VecPush, Vector, VectorSealed}};



impl<'a, T> RecursiveList for &'a Vec0<T> {
    type T = &'a T;
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

impl<'a, Inner> RecursiveList for &'a VecPush<Inner>
where
    Inner: Vector,
    &'a Inner: Vector<T = &'a Inner::T>,
{
    type T = &'a Inner::T;
    type Case = RecursiveCase;
    type Base = Vec0<Self::T>;
    type Push = VecPush<Self>;
    type Inner = &'a Inner;
    
    const BASE: Self::Base = Vec0(PhantomData);
    
    fn push(self, end: Self::T) -> Self::Push {
        VecPush { inner: self, end }
    }
    
    fn contents(self) -> <Self::Case as RecursiveListCase<Self::T, Self::Inner>>::Contents {
        RecursiveContents {
            inner: &self.inner,
            end: &self.end,
        }
    }
}

impl<'a, T> VectorSealed for &'a Vec0<T> {}
impl<'a, T> Vector for &'a Vec0<T> {}

impl<'a, Inner> VectorSealed for &'a VecPush<Inner> where Inner: Vector {}
impl<'a, Inner> Vector for &'a VecPush<Inner> where Inner: Vector, &'a Inner: Vector<T = &'a Inner::T> {}

impl<'a, T, R> Mappable<R> for &'a Vec0<T> {
    type Mapped = Vec0<R>;
}

impl<'a, Inner, R> Mappable<R> for &'a VecPush<Inner>
where
    Inner: Vector,
    &'a Inner: Mappable<R>,
    <&'a Inner as Mappable<R>>::Mapped: Vector<T = R>,
{
    type Mapped = VecPush<<&'a Inner as Mappable<R>>::Mapped>;
}

