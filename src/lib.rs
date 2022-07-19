//! # RunPack
//! TODO

#![no_std]

mod core;
mod primitives;

pub use self::core::*;

//TODO: prelude: a RunPack script with word definitions that is (optionally) executed before the user script
//TODO: plugins: crates that append lexicons to the core: string, stdio, fs, math, etc
//TODO: async words