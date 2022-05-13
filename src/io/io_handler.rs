use std::fmt::Debug;
use std::intrinsics::transmute;

use super::*;
use crate::types::*;
use crate::*;

type Result<T> = std::io::Result<T>;

/// The `IOHandler` trait allows for I/O with different sources/destinations.
///
/// `IOHandler` is a custom implementation and combination of the ([`Debug`], ) [`Read`](std::io::Read), [`Write`](std::io::Write),
/// and [`Seek`](std::io::Seek) traits.
///
/// # Implemented Types
/// This crate implentes `IOHandler` on the following types:
/// - [`FileMem`]
/// - [`FileNull`]
/// - [`std::fs::File`]
pub trait IOHandler: Debug {
    /// Pulls the exact number of bytes from this source required to fill `buf`.
    ///
    /// This is (generally) a redirection to [`Read::read_exact()`] and functions the same.
    ///
    /// For more information, see [`Read::read_exact()`].
    ///
    /// [`Read::read_exact()`]: std::io::Read::read_exact
    ///
    /// # Examples
    /// ```
    /// use std::io;
    /// use lcms2::io::{FileMem, IOHandler};
    ///
    /// fn main() -> io::Result<()> {
    ///     let mut f = FileMem::new(vec![42u8;15]);
    ///     let mut buf = [0u8; 10];
    ///
    ///     // read exactly 10 bytes
    ///     f.read(&mut buf)?;
    ///     Ok(())
    /// }
    /// ```
    fn read(&mut self, buf: &mut [u8]) -> Result<()>;

    /// Seek to an offset, in bytes, in a stream.
    ///
    /// This is (generally) a redirection to [`Seek::seek()`] and functions the same (except without a return value).
    ///
    /// For more information, see [`Seek::seek()`].
    ///
    /// [`Seek::seek()`]: std::io::Seek::seek
    ///
    /// # Examples
    /// ```
    /// use lcms2::io::{FileMem, IOHandler};
    /// use std::io::SeekFrom;
    /// 
    /// let mut buf = [42, 69, 123, 7, 255];
    /// let mut file = FileMem::new(buf.as_mut_slice());
    /// let mut b = [0];
    /// 
    /// file.seek(SeekFrom::End(-1)).unwrap(); // position == 4
    /// file.read(&mut b).unwrap(); assert_eq!(b[0], 255); // position == 5
    /// 
    /// file.seek(SeekFrom::Start(0)).unwrap(); // position == 0
    /// file.read(&mut b).unwrap(); assert_eq!(b[0], 42); // position == 1
    /// 
    /// file.seek(SeekFrom::Current(2)).unwrap(); // position == 3
    /// file.read(&mut b).unwrap(); assert_eq!(b[0], 7); // position == 4
    /// 
    /// file.seek(SeekFrom::Start(2)).unwrap(); // position == 2
    /// file.read(&mut b).unwrap(); assert_eq!(b[0], 123); // position == 3
    /// 
    /// file.seek(SeekFrom::Current(-2)).unwrap(); // position == 1
    /// file.read(&mut b).unwrap(); assert_eq!(b[0], 69); // position == 2
    /// 
    /// file.seek(SeekFrom::Start(4)).unwrap(); // position == 4
    /// file.read(&mut b).unwrap(); assert_eq!(b[0], 255); // position == 5
    /// 
    /// file.seek(SeekFrom::End(-5)).unwrap(); // position == 0
    /// file.read(&mut b).unwrap(); assert_eq!(b[0], 42); // position == 1
    /// ```
    fn seek(&mut self, pos: SeekFrom) -> Result<()>;

    /// Closes the underlying I/O mechanism by consuming and dropping itself. Implementors MUST handle any special
    /// handling upon closing I/O. [`FileMem`] and [`FileNull`] have nothing special to do when dropping, and [`File`]
    /// automatically handles it's closing via [`Drop`].
    /// 
    /// [`File`]: std::fs::File
    /// 
    /// # Examples
    /// The rust borrow checker helps tremdously in preventing the use of closed I/O objects. `close()` consumes the
    /// object, so the variable cannot be used afterwards.
    /// ```compile_fail
    /// use std::fs::File;
    /// use lcms2::io::IOHandler;
    ///
    /// let mut file = File::create("filename.ext").unwrap();
    /// file.close(); //consumes, drops, and closes the file
    ///
    /// file.write(&[0u8]).unwrap(); //fails to compile
    /// /* borrow of moved value: `file` */
    /// ```
    fn close(self) -> Result<()>;

    /// Returns the current seek position from the start of the stream.
    ///
    /// This is (generally) a redirection to [`Seek::stream_position()`] and functions the same.
    ///
    /// For more information, see [`Seek::stream_position()`].
    ///
    /// [`Seek::stream_position()`]: std::io::Seek::stream_position
    /// 
    /// # Examples
    /// ```
    /// use lcms2::io::{FileMem, IOHandler};
    /// 
    /// let mut buf = Vec::new();
    /// let mut file = FileMem::new(&mut buf);
    /// 
    /// assert_eq!(file.tell().unwrap(), 0);
    /// 
    /// file.write_u64(0).unwrap(); // 8 byte value
    /// assert_eq!(file.tell().unwrap(), 8);
    /// 
    /// file.write_u32(0).unwrap(); // 4 byte value + previous 8 = 12
    /// assert_eq!(file.tell().unwrap(), 12);
    /// 
    /// file.write_u16(0).unwrap(); // 2 byte value + previous 12 = 14
    /// assert_eq!(file.tell().unwrap(), 14);
    /// 
    /// file.write_u8(0).unwrap(); // 1 byte value + previous 14 = 15
    /// assert_eq!(file.tell().unwrap(), 15);
    /// ```
    fn tell(&mut self) -> Result<usize>;

    /// Attempts to write an entire buffer into this I/O destination.
    ///
    /// This is (generally) a redirection to [`Write::write_all()`] and functions the same.
    ///
    /// For more information, see [`Write::write_all()`].
    ///
    /// [`Write::write_all()`]: std::io::Write::write_all
    ///
    /// # Examples
    /// ```
    /// use std::io;
    /// use lcms2::io::{FileMem, IOHandler};
    ///
    /// fn main() -> io::Result<()> {
    ///     let mut buf = FileMem::new(Vec::new());
    ///
    ///     buf.write(b"some bytes")?;
    ///     Ok(())
    /// }
    /// ```
    fn write(&mut self, buf: &[u8]) -> Result<()>;

    /// Returns the length of this stream (in bytes).
    /// 
    /// This method is implemented using up to two [`seek`] and two [`tell`] operations. If this method returns
    /// successfully, the seek position is unchanged (i.e. the position before calling this method is the same as
    /// afterwards). However, if this method returns an error, the seek position is unspecified.
    fn reported_size(&mut self) -> Result<usize> {
        let current_pos = self.tell()?;
        self.seek(SeekFrom::End(0))?;
        let result = self.tell();
        self.seek(SeekFrom::Start(current_pos as u64))?;

        result
    }

    /// Reads a u8 value.
    /// 
    /// ```rust
    /// use lcms2::io::{FileMem, IOHandler};
    ///
    /// let mut buf = [42u8; 1];
    /// let mut mem = FileMem::new(buf.as_mut_slice());
    ///
    /// assert_eq!(mem.read_u8().unwrap(), 42u8);
    /// ```
    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0u8];
        self.read(&mut buf)?;

        Ok(buf[0])
    }

    /// Reads a u16 value.
    /// 
    /// ```rust
    /// use lcms2::io::{FileMem, IOHandler};
    ///
    /// let mut buf = 42u16.to_be_bytes();
    /// let mut mem = FileMem::new(buf.as_mut_slice());
    ///
    /// assert_eq!(mem.read_u16().unwrap(), 42u16);
    /// ```
    fn read_u16(&mut self) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.read(&mut buf)?;

        let value = u16::from_ne_bytes(buf);
        Ok(adjust_endianness_u16(value))
    }

    /// Reads u16 values to fill a buffer.
    /// 
    /// ```rust
    /// use lcms2::io::{FileMem, IOHandler};
    ///
    /// let mut buf = [0u8; 6];
    /// buf[0..2].copy_from_slice(&42u16.to_be_bytes());
    /// buf[2..4].copy_from_slice(&69u16.to_be_bytes());
    /// buf[4..6].copy_from_slice(&255u16.to_be_bytes());
    /// let mut mem = FileMem::new(buf.as_mut_slice());
    ///
    /// let mut read_buf = [0u16; 3];
    /// mem.read_u16_array(&mut read_buf).unwrap();
    ///
    /// assert_eq!(read_buf, [42u16, 69u16, 255u16]);
    /// ```
    fn read_u16_array(&mut self, buffer: &mut [u16]) -> Result<()> {
        for item in buffer.iter_mut() {
            *item = self.read_u16()?;
        }
        Ok(())
    }

    /// Reads a u32 value.
    /// 
    /// ```rust
    /// use lcms2::io::{FileMem, IOHandler};
    ///
    /// let mut buf = 42u32.to_be_bytes();
    /// let mut mem = FileMem::new(buf.as_mut_slice());
    ///
    /// assert_eq!(mem.read_u32().unwrap(), 42u32);
    /// ```
    fn read_u32(&mut self) -> Result<u32> {
        let mut buf = [0u8; 4];
        self.read(&mut buf)?;

        let value = u32::from_ne_bytes(buf);
        Ok(adjust_endianness_u32(value))
    }

    /// Reads a f32 value.
    /// 
    /// ```rust
    /// use lcms2::io::{FileMem, IOHandler};
    ///
    /// let mut buf = 42f32.to_be_bytes();
    /// let mut mem = FileMem::new(buf.as_mut_slice());
    ///
    /// assert_eq!(mem.read_f32().unwrap(), 42f32);
    /// ```
    fn read_f32(&mut self) -> Result<f32> {
        // read as a u32 in case magic changes values read upside down due to endianness.
        let uint_value = self.read_u32()?;

        // flip from u32 to f32
        unsafe { Ok(transmute::<u32, f32>(uint_value)) }
    }

    /// Reads a u64 value.
    /// 
    /// ```rust
    /// use lcms2::io::{FileMem, IOHandler};
    ///
    /// let mut buf = 42u64.to_be_bytes();
    /// let mut mem = FileMem::new(buf.as_mut_slice());
    ///
    /// assert_eq!(mem.read_u64().unwrap(), 42u64);
    /// ```
    fn read_u64(&mut self) -> Result<u64> {
        let mut buf = [0u8; 8];
        self.read(&mut buf)?;

        let value = u64::from_ne_bytes(buf);
        Ok(adjust_endianness_u64(value))
    }

    /// Reads a Fixed Point [Q15.16](https://en.wikipedia.org/wiki/Q_(number_format)) Number as a f64 value.
    /// 
    /// ```rust
    /// use lcms2::io::{FileMem, IOHandler};
    /// 
    /// let mut buf = 0x0002_8000u32.to_be_bytes(); // 0x0002_8000 == 2.5 in s15f16 fixed point
    /// let mut mem = FileMem::new(buf.as_mut_slice());
    /// 
    /// assert_eq!(mem.read_s15f16().unwrap(), 2.5);
    /// ```
    fn read_s15f16(&mut self) -> Result<f64> {
        let fixed_point = unsafe { transmute::<u32, S15F16>(self.read_u32()?) };
        Ok(s15f16_to_f64(fixed_point))
    }

    /// Reads a CIEXYZ value stored as 3 Fixed Point [Q15.16](https://en.wikipedia.org/wiki/Q_(number_format)) Numbers.
    /// 
    /// ```rust
    /// use lcms2::io::{FileMem, IOHandler};
    /// use lcms2::types::CIEXYZ;
    /// 
    /// let mut buf = [0x00, 0x02, 0x80, 0x00,  // 0x0002_8000 == 2.5 in s15f16 fixed point
    ///                0x10, 0x00, 0x20, 0x00,  // 0x1000_2000 == 4096.125
    ///                0xFF, 0xFE, 0xC0, 0x00]; // 0xFFFE_C000 == -1.25
    /// let mut mem = FileMem::new(buf.as_mut_slice());
    /// 
    /// assert_eq!(mem.read_xyz().unwrap(), CIEXYZ { X: 2.5, Y: 4096.125, Z: -1.25 });
    /// ```
    fn read_xyz(&mut self) -> Result<CIEXYZ> {
        let x = self.read_s15f16()?;
        let y = self.read_s15f16()?;
        let z = self.read_s15f16()?;

        Ok(CIEXYZ { X: x, Y: y, Z: z })
    }

    /// Writes a u8 value.
    /// 
    /// ```rust
    /// use lcms2::io::{FileMem, IOHandler};
    ///
    /// let mut buf = [0; 1];
    /// let mut mem = FileMem::new(buf.as_mut_slice());
    /// 
    /// mem.write_u8(42).unwrap();
    ///
    /// assert_eq!(buf[0], 42);
    /// ```
    fn write_u8(&mut self, value: u8) -> Result<()> {
        self.write(&[value])
    }

    /// Writes a u16 value.
    /// 
    /// ```rust
    /// use lcms2::io::{FileMem, IOHandler};
    /// 
    /// # fn adjust_endianness(buf: [u8; 2]) -> u16 {
    /// #     u16::from_ne_bytes(lcms2::io::adjust_endianness_16(buf))
    /// # }
    ///
    /// let mut buf = [0; 2];
    /// let mut mem = FileMem::new(buf.as_mut_slice());
    /// 
    /// mem.write_u16(42).unwrap();
    ///
    /// assert_eq!(adjust_endianness(buf), 42);
    /// ```
    fn write_u16(&mut self, value: u16) -> Result<()> {
        let value = adjust_endianness_u16(value);
        self.write(&value.to_ne_bytes())
    }

    /// Writes all u16 values from a buffer.
    /// 
    /// ```rust
    /// use lcms2::io::{FileMem, IOHandler};
    /// 
    /// # fn adjust_endianness(buf: &[u8]) -> u16 {
    /// #     let val = [buf[0], buf[1]];
    /// #     u16::from_ne_bytes(lcms2::io::adjust_endianness_16(val))
    /// # }
    ///
    /// let mut buf = [0; 6];
    /// let mut mem = FileMem::new(buf.as_mut_slice());
    /// 
    /// mem.write_u16_array(&[42, 69, 255]).unwrap();
    ///
    /// assert_eq!(adjust_endianness(&buf[0..2]), 42);
    /// assert_eq!(adjust_endianness(&buf[2..4]), 69);
    /// assert_eq!(adjust_endianness(&buf[4..6]), 255);
    /// ```
    fn write_u16_array(&mut self, buffer: &[u16]) -> Result<()> {
        for value in buffer.iter() {
            self.write_u16(*value)?;
        }
        Ok(())
    }

    /// Writes a u32 value.
    /// 
    /// ```rust
    /// use lcms2::io::{FileMem, IOHandler};
    /// 
    /// # fn adjust_endianness(buf: [u8; 4]) -> u32 {
    /// #     u32::from_ne_bytes(lcms2::io::adjust_endianness_32(buf))
    /// # }
    ///
    /// let mut buf = [0; 4];
    /// let mut mem = FileMem::new(buf.as_mut_slice());
    /// 
    /// mem.write_u32(42).unwrap();
    ///
    /// assert_eq!(adjust_endianness(buf), 42);
    /// ```
    fn write_u32(&mut self, value: u32) -> Result<()> {
        let value = adjust_endianness_u32(value);
        self.write(&value.to_ne_bytes())
    }

    /// Writes a u32 value.
    /// 
    /// ```rust
    /// use lcms2::io::{FileMem, IOHandler};
    /// 
    /// # fn adjust_endianness(buf: [u8; 4]) -> f32 {
    /// #     f32::from_ne_bytes(lcms2::io::adjust_endianness_32(buf))
    /// # }
    ///
    /// let mut buf = [0; 4];
    /// let mut mem = FileMem::new(buf.as_mut_slice());
    /// 
    /// mem.write_f32(42.0).unwrap();
    ///
    /// assert_eq!(adjust_endianness(buf), 42.0);
    /// ```
    fn write_f32(&mut self, value: f32) -> Result<()> {
        // flip from f32 to u32
        let uint_value = unsafe { transmute::<f32, u32>(value) };

        self.write_u32(uint_value)
    }

    /// Writes a u64 value.
    /// 
    /// ```rust
    /// use lcms2::io::{FileMem, IOHandler};
    /// 
    /// # fn adjust_endianness(buf: [u8; 8]) -> u64 {
    /// #     u64::from_ne_bytes(lcms2::io::adjust_endianness_64(buf))
    /// # }
    ///
    /// let mut buf = [0; 8];
    /// let mut mem = FileMem::new(buf.as_mut_slice());
    /// 
    /// mem.write_u64(42).unwrap();
    ///
    /// assert_eq!(adjust_endianness(buf), 42);
    /// ```
    fn write_u64(&mut self, value: u64) -> Result<()> {
        let value = adjust_endianness_u64(value);
        self.write(&value.to_ne_bytes())
    }

    /// Writes a Fixed Point [Q15.16](https://en.wikipedia.org/wiki/Q_(number_format)) Number as a f64 value.
    /// 
    /// ```rust
    /// use lcms2::io::{FileMem, IOHandler};
    ///
    /// let mut buf = [0; 4];
    /// let mut mem = FileMem::new(buf.as_mut_slice());
    /// 
    /// mem.write_s15f16(2.5).unwrap();
    ///
    /// assert_eq!(buf, [0x00, 0x02, 0x80, 0x00]); // 0x0002_8000 == 2.5 in s15f16 fixed point
    /// ```
    fn write_s15f16(&mut self, value: f64) -> Result<()> {
        let fixed_point = f64_to_s15f16(value);
        self.write_u32(unsafe { transmute::<i32, u32>(fixed_point) })
    }

    /// Writes a CIEXYZ value stored as 3 Fixed Point [Q15.16](https://en.wikipedia.org/wiki/Q_(number_format)) Numbers.
    /// 
    /// ```rust
    /// use lcms2::io::{FileMem, IOHandler};
    /// use lcms2::types::CIEXYZ;
    ///
    /// let mut buf = [0; 12];
    /// let mut mem = FileMem::new(buf.as_mut_slice());
    /// 
    /// mem.write_xyz(CIEXYZ { X: 2.5, Y: 4096.125, Z: -1.25 }).unwrap();
    ///
    /// assert_eq!(buf, [0x00, 0x02, 0x80, 0x00,   // 0x0002_8000 == 2.5 in s15f16 fixed point
    ///                  0x10, 0x00, 0x20, 0x00,   // 0x1000_2000 == 4096.125
    ///                  0xFF, 0xFE, 0xC0, 0x00]); // 0xFFFE_C000 == -1.25);
    /// ```
    fn write_xyz(&mut self, value: CIEXYZ) -> Result<()> {
        self.write_s15f16(value.X)?;
        self.write_s15f16(value.Y)?;
        self.write_s15f16(value.Z)
    }
}

fn s15f16_to_f64(value: S15F16) -> f64 {
    let sign = if value < 0 { -1.0 } else { 1.0 };
    let value = value.abs();

    let whole = ((value >> 16) & 0xFFFF) as u16;
    let frac_part = (value & 0xFFFF) as u16;

    let mid = frac_part as f64 / 65536.0;
    let floater = whole as f64 + mid;

    return sign * floater;
}

fn f64_to_s15f16(value: f64) -> S15F16 {
    ((value * 65536.0) + 0.5).floor() as S15F16
}
