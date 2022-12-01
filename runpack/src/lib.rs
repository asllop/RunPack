//! # RunPack
//! TODO

#![no_std]

#[macro_use]
extern crate alloc;

mod core;
mod primitives;
mod prelude;

pub use self::core::*;
pub use self::primitives::register_primitives;

//TODO: tests