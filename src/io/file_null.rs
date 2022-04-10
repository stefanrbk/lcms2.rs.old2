use std::io::{Read, Result};

pub struct FileNull {
    pointer: usize,
}

impl Read for FileNull {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        todo!()
    }
}
