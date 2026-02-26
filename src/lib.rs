#![deny(unsafe_code)]

pub mod number;
pub mod static_list;
pub mod vector;
// pub mod matrix;

pub use number::*;
pub use static_list::*;
pub use vector::*;
// pub use matrix::*;


#[cfg(test)] mod test;
