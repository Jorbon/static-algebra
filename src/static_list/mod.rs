//! Trait describing a static list of values, including static indexing and iteration

pub mod iterable;
pub mod static_index;

use crate::number::Number;


/// Abstract trait describing a static list of values.
/// The sub-trait [`RecursiveList`] provides the actual trait bound implementations for use elsewhere in the crate.
pub trait StaticList<T> {
    type Length: Number;
}

pub trait StaticListMut<T>:
    StaticList<T>
{}

pub trait StaticListOwned<T>:
    StaticList<T>
{}

