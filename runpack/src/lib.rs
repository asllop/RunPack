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

//TODO: plugins: crates that append lexicons to the core: string, stdio, fs, math, etc
//TODO: async words
//TODO: tests