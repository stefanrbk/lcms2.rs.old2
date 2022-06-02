use std::io::{Result, SeekFrom};

use super::IOHandler;

/// A `FileNull` object is a place to read from or write into nothing.
/// 
/// This works similar to [`Empty`] and [`Sink`], but still behaves like the other I/O devices.
/// 
/// [`Empty`]: std::io::Empty
/// [`Sink`]: std::io::Sink
/// 
/// #Examples
/// ```rust
/// use lcms2::io::{FileNull, IOHandler};
/// use std::io::SeekFrom;
/// 
/// let mut buf = [0];
/// let mut file = FileNull::new();
/// 
/// // file position advances as you would expect
/// assert_eq!(file.tell().unwrap(), 0);
/// file.write(&[42, 69, 123, 7, 255]).unwrap();
/// assert_eq!(file.tell().unwrap(), 5);
/// 
/// // regardless of what is written into a FileNull, '0' is always read out
/// file.seek(SeekFrom::Start(0)).unwrap();
/// assert_eq!(file.read_u32().unwrap(), 0);
/// ```
#[derive(Debug)]
pub struct FileNull {
    pub(crate) pointer: usize,
    pub(crate) used_space: usize,
}

impl FileNull {
    /// Creates a new `FileNull` object
    /// 
    /// #Examples
    /// ```rust
    /// use lcms2::io::FileNull;
    /// 
    /// let file = FileNull::new();
    /// ```
    pub const fn new() -> FileNull {
        FileNull { pointer: 0, used_space: 0 }
    }
}

impl Default for FileNull {
    fn default() -> Self {
        Self::new()
    }
}

impl IOHandler for FileNull {
    fn read(&mut self, buf: &mut [u8]) -> Result<()> {
        let len = buf.len();
        self.pointer += len;

        Ok(())
    }

    fn seek(&mut self, pos: SeekFrom) -> Result<()> {
        match pos {
            SeekFrom::Start(value) => {
                self.pointer = value as usize;
            },
            SeekFrom::End(value) => {
                self.pointer = self.used_space + value as usize;
            },
            SeekFrom::Current(value) => {
                self.pointer += value as usize;
            }
        }
        Ok(())
    }

    fn close(self) -> Result<()> {
        Ok(())
    }

    fn tell(&mut self) -> Result<usize> {
        Ok(self.pointer)
    }

    fn write(&mut self, buf: &[u8]) -> Result<()> {
        let len = buf.len();
        self.pointer += len;

        if self.pointer > self.used_space {
            self.used_space = self.pointer
        }

        Ok(())
    }
}
