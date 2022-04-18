use std::io::{Result, SeekFrom};

use super::IOHandler;

pub struct FileNull {
    pub(crate) pointer: usize,
    pub(crate) used_space: usize,
}

impl FileNull {
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

    fn close(&mut self) -> Result<()> {
        Ok(())
    }

    fn tell(&mut self) -> usize {
        self.pointer
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
