use std::{
    convert::AsRef,
    fmt::Debug,
    io::{Cursor, Read, Result, Seek, Write},
};

use super::IOHandler;

/// A `FileMem` object wraps an in-memory buffer to provide I/O functionality.
///
/// `FileMem` objects are essentially wrappers for any valid [`Cursor`](Cursor) which implement
/// [`Write`](Write). A valid [`Cursor`] wraps anything implementing <code>[AsRef]<\[u8]></code>.
///
/// # Implemented Types
/// The standard library implements [`Write`] on the following [`Cursor`] types:
/// - <code>Cursor<[&mut \[u8\]][bytes]></code>
/// - <code>Cursor<&mut [Vec]\<u8>></code>
/// - <code>Cursor<[Vec]\<u8>></code>
/// - <code>Cursor<[Box]\<\[u8\]>></code>
///
/// [bytes]: std::slice "slice"
///
/// # Examples
/// `FileMem` can be used like any other "`File`" type.
/// ```
/// use lcms2::io::{FileMem, IOHandler};
/// use std::io::SeekFrom;
///
/// let mut buf = [0u8; 15];
///
/// let mut file = FileMem::new(buf.as_mut_slice());
///
/// file.seek(SeekFrom::Start(10)).unwrap();
/// file.write_u8(42).unwrap();
///
/// assert_eq!(buf[10], 42);
/// ```
#[derive(Debug)]
pub struct FileMem<T>
where
    T: AsRef<[u8]>,
    Cursor<T>: Write,
{
    pub(crate) cursor: Cursor<T>,
}

impl<T> FileMem<T>
where
    T: AsRef<[u8]>,
    Cursor<T>: Write,
{
    /// Creates a new `FileMem` object with the provided in-memory buffer.
    /// 
    /// As with [`Cursor`], the initial position is `0` even if the underlying buffer (e.g., [`Vec`]) is not empty.
    /// Writing to a `FileMem` object starts with overwriting [`Vec`] content, not with appending to it.
    /// 
    /// # Examples
    /// ```
    /// use lcms2::io::FileMem;
    /// 
    /// let buf = FileMem::new(Vec::new());
    /// # fn force_inference(_: &FileMem<Vec<u8>>) {}
    /// # force_inference(&buf);
    /// ```
    pub fn new(buf: T) -> FileMem<T> {
        FileMem {
            cursor: Cursor::new(buf),
        }
    }
}

impl<T> IOHandler for FileMem<T>
where
    T: AsRef<[u8]> + Debug,
    Cursor<T>: Write,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<()> {
        self.cursor.read_exact(buf)
    }

    fn seek(&mut self, pos: std::io::SeekFrom) -> Result<()> {
        self.cursor.seek(pos)?;
        Ok(())
    }

    fn close(self) -> Result<()> {
        Ok(())
    }

    fn tell(&mut self) -> Result<usize> {
        Ok(self.cursor.position() as usize)
    }

    fn write(&mut self, buf: &[u8]) -> Result<()> {
        self.cursor.write_all(buf)
    }
}
