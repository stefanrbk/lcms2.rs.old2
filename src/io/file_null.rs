use std::io::{Read, Result, Write, Seek};

use super::Stream;

pub struct FileNull {
    pub(crate) pointer: usize,
    pub(crate) used_space: usize,
}

impl Read for FileNull {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let len = buf.len();
        self.pointer += len;

        Ok(len)
    }
}
impl Write for FileNull {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let len = buf.len();
        self.pointer += len;

        if self.pointer > self.used_space {
            self.used_space = self.pointer
        }

        Ok(len)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
impl Seek for FileNull {
    fn seek(&mut self, pos: std::io::SeekFrom) -> Result<u64> {
        match pos {
            std::io::SeekFrom::Start(value) => {
                self.pointer = value as usize;

                Ok(value)
            },
            std::io::SeekFrom::End(value) => {
                self.pointer = self.used_space + value as usize;

                Ok(self.pointer as u64)
            },
            std::io::SeekFrom::Current(value) => {
                self.pointer += value as usize;

                Ok(self.pointer as u64)},
        }
    }
}
impl Stream for FileNull {}
