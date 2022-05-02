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
//! Endianness is an important concept to understand and be aware of when handling any kind of I/O. The
//! [`little_endian`] and [`big_endian`] modules assist in converting data from one format to another. Each module has
//! the same functions, just different operations depending on which format you are intending on reading or writing into
//! the format used by the CPU.
//! 
//! Since the main purpose of this library is for handling ICC profiles (which are supposed to be in big endian format),
//! the [`big_endian`] module's functions are re-exported into this module to make it easier to distinguish which
//! functions to use. This library does have an optional feature `use_little_endian` which swaps the re-exported
//! functions to the [`little_endian`] module's versions if needed.

mod io_handler;
pub mod little_endian;
pub mod big_endian;
mod file_null;
mod file_mem;
mod file_file;

use std::io::SeekFrom;

pub use io_handler::IOHandler;
pub use file_null::FileNull;
pub use file_mem::FileMem;

#[cfg(feature = "use_little_endian")]
pub use little_endian::adjust_endianness_16;
#[cfg(feature = "use_little_endian")]
pub use little_endian::adjust_endianness_32;
#[cfg(feature = "use_little_endian")]
pub use little_endian::adjust_endianness_64;
#[cfg(feature = "use_little_endian")]
pub use little_endian::adjust_endianness_u16;
#[cfg(feature = "use_little_endian")]
pub use little_endian::adjust_endianness_u32;
#[cfg(feature = "use_little_endian")]
pub use little_endian::adjust_endianness_u64;


#[cfg(not(feature = "use_little_endian"))]
pub use big_endian::adjust_endianness_16;
#[cfg(not(feature = "use_little_endian"))]
pub use big_endian::adjust_endianness_32;
#[cfg(not(feature = "use_little_endian"))]
pub use big_endian::adjust_endianness_64;
#[cfg(not(feature = "use_little_endian"))]
pub use big_endian::adjust_endianness_u16;
#[cfg(not(feature = "use_little_endian"))]
pub use big_endian::adjust_endianness_u32;
#[cfg(not(feature = "use_little_endian"))]
pub use big_endian::adjust_endianness_u64;

pub enum AccessMode {
    Read,
    Write,
}
