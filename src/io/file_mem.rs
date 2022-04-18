use std::{
    convert::AsRef,
    io::{Cursor, Result, Read, Seek, Write},
};

use super::IOHandler;

pub struct FileMem<T>
where
    T: AsRef<[u8]>,
    Cursor<T>: Write,
{
    pub(crate) cursor: Cursor<T>,
}

impl<T> IOHandler for FileMem<T>
where
    T: AsRef<[u8]>,
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

    fn tell(&mut self) -> usize {
        self.cursor.position() as usize
    }

    fn write(&mut self, buf: &[u8]) -> Result<()> {
        self.cursor.write_all(buf)
    }
}
