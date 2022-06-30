//! Traits, helpers, and type definitions for I/O functionality.
//! 
//! The `lcms2::io` module contains a number of common things you'll need when doing input and output with this library.
//! The most core part of this module is the [`IOHandler`] trait, which standardizes the forms of I/O for the library.
//! 
//! # IOHandler
//! 
//! [`IOHandler`] is essentially a big wrapper for the [`Read`](std::io::Read), [`Write`](std::io::Write), and
//! [`Seek`](std::io::Seek) traits, along with some helper functions.
//! 
//! # I/O Sources
//! 
//! This module contains 2 different types representing different I/O sources implementing the [`IOHandler`] trait and 1 
//! retroactive implementation of an existing type. They are:
//! - [`FileMem`]
//! - [`FileNull`]
//! - [`std::fs::File`]
//! 
//! # Endianness Helper Functions
//! 
//! ICC profiles are to be in big endian format. Since most CPUs are little endian these days, conversion functions are
//! included to handle the conversions.

mod io_handler;
mod file_null;
mod file_mem;
mod file_file;

use std::io::SeekFrom;

pub use io_handler::IOHandler;
pub use file_null::FileNull;
pub use file_mem::FileMem;

/// Convert a `[u8; 2]` array to/from big endian.
#[inline(always)]
pub fn adjust_endianness_16(value: [u8; 2]) -> [u8; 2] {
    let value = u16::from_be_bytes(value);
    value.to_ne_bytes()
}

/// Convert a `[u8; 4]` array to/from big endian.
#[inline(always)]
pub fn adjust_endianness_32(value: [u8; 4]) -> [u8; 4] {
    let value = u32::from_be_bytes(value);
    value.to_ne_bytes()
}

/// Convert a `[u8; 8]` array to/from big endian.
#[inline(always)]
pub fn adjust_endianness_64(value: [u8; 8]) -> [u8; 8] {
    let value = u64::from_be_bytes(value);
    value.to_ne_bytes()
}

/// Convert a [`u16`] to/from big endian.
#[inline(always)]
pub fn adjust_endianness_u16(value: u16) -> u16 {
    value.to_be()
}

/// Convert a [`u32`] to/from big endian.
#[inline(always)]
pub fn adjust_endianness_u32(value: u32) -> u32 {
    value.to_be()
}

/// Convert a [`u64`] to/from big endian.
#[inline(always)]
pub fn adjust_endianness_u64(value: u64) -> u64 {
    value.to_be()
}

/// Enumeration of possible I/O actions
pub enum AccessMode {
    /// Open in `Read` mode.
    Read,
    /// Open in `Write` mode.
    Write,
}
