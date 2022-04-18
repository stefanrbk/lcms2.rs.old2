use std::{fs::File, io::{Result, Read, Seek, Write}};

use super::IOHandler;

impl IOHandler for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<()> {
        self.read_exact(buf)
    }

    fn seek(&mut self, pos: std::io::SeekFrom) -> Result<()> {
        Seek::seek(self, pos)?;
        Ok(())
    }

    fn close(&mut self) -> Result<()> {
        self.sync_data()
    }

    fn tell(&mut self) -> Result<usize> {
        Ok(self.stream_position()? as usize)
    }

    fn write(&mut self, buf: &[u8]) -> Result<()> {
        self.write_all(buf)
    }
}
