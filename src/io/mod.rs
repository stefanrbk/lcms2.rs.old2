mod io_handler;
mod little_endian;
mod big_endian;
mod file_null;

use std::io::{Write, Seek, Read, SeekFrom, Result};

pub use io_handler::IOHandler;
pub use file_null::FileNull;

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

pub trait Stream: Read + Write + Seek {}
pub trait IOStream {
    fn read(&mut self, iohandler: &mut IOHandler, buf: &mut [u8]) -> Result<()>;
    fn seek(&mut self, iohandler: &mut IOHandler, pos: SeekFrom) -> Result<()>;
    fn close(&mut self, iohandler: &mut IOHandler) -> Result<()>;
    fn tell(&mut self, iohandler: &mut IOHandler) -> usize;
    fn write(&mut self, iohandler: &mut IOHandler, buf: &[u8]) -> Result<()>;
}
