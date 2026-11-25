use core::marker::PhantomData;

use crate::{recursive_list::{BaseCase, BaseContents, RecursiveCase, RecursiveContents, RecursiveList, RecursiveListCase}, static_list::Mappable, vector::{Vec0, VecPush, Vector, VectorSealed}};



impl<'a, T> RecursiveList for &'a mut Vec0<T> {
    type T = &'a mut T;
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

impl<'a, Inner> RecursiveList for &'a mut VecPush<Inner>
where
    Inner: Vector,
    &'a mut Inner: Vector<T = &'a mut Inner::T>,
{
    type T = &'a mut Inner::T;
    type Case = RecursiveCase;
    type Base = Vec0<Self::T>;
    type Push = VecPush<Self>;
    type Inner = &'a mut Inner;
    
    const BASE: Self::Base = Vec0(PhantomData);
    
    fn push(self, end: Self::T) -> Self::Push {
        VecPush { inner: self, end }
    }
    
    fn contents(self) -> <Self::Case as RecursiveListCase<Self::T, Self::Inner>>::Contents {
        RecursiveContents {
            inner: &mut self.inner,
            end: &mut self.end,
        }
    }
}

impl<'a, T> VectorSealed for &'a mut Vec0<T> {}
impl<'a, T> Vector for &'a mut Vec0<T> {}

impl<'a, Inner> VectorSealed for &'a mut VecPush<Inner> where Inner: Vector {}
impl<'a, Inner> Vector for &'a mut VecPush<Inner> where Inner: Vector, &'a mut Inner: Vector<T = &'a mut Inner::T> {}

impl<'a, T, R> Mappable<R> for &'a mut Vec0<T> {
    type Mapped = Vec0<R>;
}

impl<'a, Inner, R> Mappable<R> for &'a mut VecPush<Inner>
where
    Inner: Vector,
    &'a mut Inner: Mappable<R>,
    <&'a mut Inner as Mappable<R>>::Mapped: Vector<T = R>,
{
    type Mapped = VecPush<<&'a mut Inner as Mappable<R>>::Mapped>;
}

