// third party library.
extern crate num as libnum;

// common public types
pub type TNT = f32;

// macros should be at the top in order for macros to be accessible in subsequent modules
#[macro_use]
pub mod macros;

pub mod vector;
pub mod stack;
pub mod runtime;

mod base;
mod math;
mod faust;
