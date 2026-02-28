#![no_std]
#![deny(unsafe_code)]

pub mod iterable;
pub mod number;
pub mod static_list;
pub mod vector;
pub mod matrix;

pub use iterable::*;
pub use number::*;
pub use static_list::*;
pub use vector::*;
pub use matrix::*;

#[cfg(feature = "num-traits")]
pub mod num_traits;
#[cfg(feature = "num-traits")]
#[allow(unused_imports)]
pub use num_traits::*;

#[cfg(test)] mod test;
