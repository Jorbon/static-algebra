//! Algebraic vector type. Specialization of [`StaticList`] to include vector operations.

pub mod owned_impl;
pub mod ref_impl;
pub mod ref_mut_impl;
// pub mod view;
pub mod helper;
mod from;
// mod fmt;
pub mod ops;
mod core_ops;

use core::marker::PhantomData;

use crate::{recursive_list::RecursiveList, static_list::Iterable};


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

