//! Algebraic vector type. Specialization of [`StaticList`] to include vector operations.

// pub mod view;
// pub mod helper;
// mod from;
// mod fmt;
// mod ops;
// mod core_ops;

use crate::{recursive_list::{BaseCase, BaseContents, RecursiveCase, RecursiveContents, RecursiveList, RecursiveListCase}, static_list::Iterable};


trait VectorSealed {}
#[allow(private_bounds)]
pub trait Vector<T>:
    VectorSealed + RecursiveList<T> + Iterable<T>
{}


#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Vec0;

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct VecPush<T, Inner>
where
    Inner: RecursiveList<T>,
{
    pub(crate) inner: Inner,
    pub(crate) end: T,
}

// impl<T, Inner> VecPush<T, Inner>
// where
//     Inner: Vector<T>,
// {
//     #[inline]
//     pub const fn push(inner: Inner, end: T) -> Self {
//         Self { inner, end }
//     }
// }


impl<T> RecursiveList<T> for Vec0 {
    type Case = BaseCase;
    type Same<U> = Vec0;
    type Base = Vec0;
    type Push = VecPush<T, Self>;
    type Inner = Vec0;
    
    const BASE: Self::Base = Vec0;
    
    fn push(self, end: T) -> Self::Push {
        VecPush { inner: self, end }
    }
    
    fn contents(self) -> <Self::Case as RecursiveListCase<T, Self::Inner>>::Contents {
        BaseContents
    }
}


// <Inner::Same<U> as RecursiveList<U>>::Push
// VecPush<U, Inner::Same<U>>

impl<T, Inner> RecursiveList<T> for VecPush<T, Inner>
where
    Inner: RecursiveList<T, Base = Vec0, Same<T> = Inner>,
{
    type Case = RecursiveCase;
    type Same<U> = VecPush<U, Inner::Same<U>>;
    type Base = Vec0;
    type Push = VecPush<T, Self>;
    type Inner = Inner;
    
    const BASE: Self::Base = Vec0;
    
    fn push(self, end: T) -> Self::Push {
        VecPush { inner: self, end }
    }
    
    fn contents(self) -> <Self::Case as RecursiveListCase<T, Self::Inner>>::Contents {
        RecursiveContents {
            inner: self.inner,
            end: self.end,
        }
    }
}

