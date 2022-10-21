//! The vector module.
//!
//! Currently contains all code
//! relating to the vector linear algebra struct.

mod impl_ops;
mod impl_vec;
mod utils;

/// The Vector struct.
///
/// Can be instantiated with any type.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Vector<T> {
    size: usize,
    data: Vec<T>,
}
