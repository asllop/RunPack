//! # RunPack
//! TODO

#![no_std]

mod core;
mod primitives;
mod prelude;

pub use self::core::*;
pub use self::primitives::register_primitives;

//TODO: plugins: crates that append lexicons to the core: string, stdio, fs, math, etc
//TODO: async words
//YODO: tests