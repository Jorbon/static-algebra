#![no_std]
#![deny(unsafe_code)]
// #![recursion_limit = "256"]

pub mod number;
pub mod conjugate;
pub mod static_list;
pub mod recursive_list;
pub mod vector;
// pub mod matrix;

// #[cfg(feature = "num-traits")] mod num_traits;

#[cfg(test)] mod test;
