#![deny(unsafe_code)]

pub mod count;
pub mod const_iterator;
pub mod vector;
// pub mod matrix;

pub use count::*;
pub use const_iterator::*;
pub use vector::*;
// pub use matrix::*;


#[cfg(test)] mod test;