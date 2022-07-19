//! # RunPack
//! TODO

#![no_std]

mod core;
mod primitives;
mod prelude;

pub use self::core::*;

//TODO: plugins: crates that append lexicons to the core: string, stdio, fs, math, etc
//TODO: async words