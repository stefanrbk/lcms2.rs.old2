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

    fn close(self) -> Result<()> {
        self.sync_data()?;
        drop(self);
        Ok(())
    }

    fn tell(&mut self) -> Result<usize> {
        Ok(self.stream_position()? as usize)
    }

    fn write(&mut self, buf: &[u8]) -> Result<()> {
        self.write_all(buf)
    }
}

#[cfg(test)]
mod test {
    use std::{io::{self, Read, Write, SeekFrom}, fs::File};

    use crate::{testing::get_temp_file_path, io::IOHandler};
    use test_case::test_case;

    #[test]
    fn test_file_tell_reports_correct_location() -> io::Result<()> {
        let filename = get_temp_file_path("test_file_tell_reports_correct_location");
        let mut file = File::create(filename)?;

        assert_eq!(file.tell()?, 0usize);

        std::io::Write::write(&mut file, &0u64.to_be_bytes())?;
        assert_eq!(file.tell()?, 8usize);

        std::io::Write::write(&mut file, &0u32.to_be_bytes())?;
        assert_eq!(file.tell()?, 12usize);

        std::io::Write::write(&mut file, &0u16.to_be_bytes())?;
        assert_eq!(file.tell()?, 14usize);

        std::io::Write::write(&mut file, &0u8.to_be_bytes())?;
        assert_eq!(file.tell()?, 15usize);

        Ok(())
    }
    #[test_case("42u64", &42u64.to_be_bytes(); "42u64")]
    #[test_case("69u32", &69u32.to_be_bytes(); "69u32")]
    #[test_case("123u16", &123u16.to_be_bytes(); "123u16")]
    #[test_case("7u8", &7u8.to_be_bytes(); "7u8")]
    fn test_file_writes_data_to_disk(n: &str, buf: &[u8]) -> io::Result<()> {
        let filename = get_temp_file_path(format!("test_file_writes_data_to_disk-{}", n).as_str());
        let handler: &mut dyn IOHandler = &mut (File::create(filename.clone())?);
        
        handler.write(buf)?;

        let mut read_buf = vec![0u8; buf.len()];
        let mut file = File::open(filename)?;

        file.read_exact(read_buf.as_mut_slice())?;

        assert_eq!(buf, read_buf.as_slice());
        
        Ok(())
    }
    #[test_case("42u64", &42u64.to_be_bytes(); "42u64")]
    #[test_case("69u32", &69u32.to_be_bytes(); "69u32")]
    #[test_case("123u16", &123u16.to_be_bytes(); "123u16")]
    #[test_case("7u8", &7u8.to_be_bytes(); "7u8")]
    fn test_file_reads_data_from_disk(n: &str, buf: &[u8]) -> io::Result<()> {
        let filename = get_temp_file_path(format!("test_file_reads_data_from_disk-{}", n).as_str());
        let mut file = File::create(filename.clone())?;

        file.write_all(buf)?;
        
        let mut read_buf = vec![0u8; buf.len()];
        let handler: &mut dyn IOHandler = &mut (File::open(filename)?);

        handler.read(read_buf.as_mut_slice())?;

        assert_eq!(buf, read_buf.as_slice());

        Ok(())
    }
    #[test]
    fn test_file_seeks_to_the_correct_location() -> io::Result<()> {
        let filename = get_temp_file_path("test_file_seeks_to_the_correct_location");
        let mut file = File::create(filename.clone())?;

        file.write_all(&[42u8, 69u8, 123u8, 7u8, 255u8])?;
        file.close()?;

        let mut handler = File::open(filename)?;
        let mut buf = [0u8];

        handler.seek(SeekFrom::End(-1))?;
        handler.read_exact(&mut buf)?;

        assert_eq!(buf[0], 255u8);

        handler.seek(SeekFrom::Start(0))?;
        handler.read_exact(&mut buf)?;

        assert_eq!(buf[0], 42u8);

        handler.seek(SeekFrom::Current(2))?;
        handler.read_exact(&mut buf)?;

        assert_eq!(buf[0], 7u8);

        handler.seek(SeekFrom::Start(2))?;
        handler.read_exact(&mut buf)?;

        assert_eq!(buf[0], 123u8);

        handler.seek(SeekFrom::Current(-2))?;
        handler.read_exact(&mut buf)?;

        assert_eq!(buf[0], 69u8);

        handler.seek(SeekFrom::Start(4))?;
        handler.read_exact(&mut buf)?;

        assert_eq!(buf[0], 255u8);

        handler.seek(SeekFrom::End(-5))?;
        handler.read_exact(&mut buf)?;

        assert_eq!(buf[0], 42u8);

        Ok(())
    }
}
