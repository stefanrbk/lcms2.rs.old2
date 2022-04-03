#![allow(dead_code)]

macro_rules! use_big_endian {
    () => {
        !cfg!(feature = "use_little_endian")
    };
}

// Fixed Point Types

pub type U8F8 = u16;
pub type S15F16 = i32;
pub type U16F16 = u32;

pub mod plugins;
pub mod state;
pub mod types;
