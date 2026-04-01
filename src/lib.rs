#![no_std]
#![deny(unsafe_code)]

pub mod iterable;
pub mod number;
pub mod static_list;
pub mod vector;
pub mod matrix;
pub mod ops;

#[cfg(feature = "num-traits")]
mod num_traits;

#[cfg(test)] mod test;
