#![deny(unsafe_code)]

pub mod count;
pub mod vector;
// pub mod matrix;

pub use count::*;
pub use vector::*;
// pub use matrix::*;


mod test;