use std::{
    convert::AsRef,
    io::{Cursor, Read, Result, Seek, Write},
};

use super::IOHandler;

pub struct FileMem<T>
where
    T: AsRef<[u8]> + Clone,
    Cursor<T>: Write,
{
    pub(crate) cursor: Cursor<T>,
}

impl<T> FileMem<T>
where
    T: AsRef<[u8]> + Clone,
    Cursor<T>: Write,
{
    pub fn new(buf: T, mode: AccessMode) -> FileMem<T>
    {
        if let AccessMode::Read = mode {
            let buf = buf.clone();
            FileMem {
                cursor: Cursor::new(buf),
            }
        } else {
            FileMem {
                cursor: Cursor::new(buf),
            }
        }
    }
}

impl<T> IOHandler for FileMem<T>
where
    T: AsRef<[u8]> + Clone,
    Cursor<T>: Write,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<()> {
        self.cursor.read_exact(buf)
    }

    fn seek(&mut self, pos: std::io::SeekFrom) -> Result<()> {
        self.cursor.seek(pos)?;
        Ok(())
    }

    fn close(&mut self) -> Result<()> {
        Ok(())
    }

    fn tell(&mut self) -> Result<usize> {
        Ok(self.cursor.position() as usize)
    }

    fn write(&mut self, buf: &[u8]) -> Result<()> {
        self.cursor.write_all(buf)
    }
}
pub enum AccessMode {
    Read,
    Write,
}
