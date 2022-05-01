use std::fmt::Debug;
use std::intrinsics::transmute;

use super::*;
use crate::types::*;
use crate::*;

type Result<T> = std::io::Result<T>;

pub trait IOHandler : Debug {
    fn read(&mut self, buf: &mut [u8]) -> Result<()>;
    fn seek(&mut self, pos: SeekFrom) -> Result<()>;

    /// ```compile_fail
    /// use std::fs::File;
    /// use lcms2::io::IOHandler;
    /// 
    /// let mut file = File::create("filename.ext").unwrap();
    /// file.close(); //consumes, drops, and closes the file
    /// 
    /// file.write(&[0u8]).unwrap(); //fails to compile
    /// ```
    fn close(self) -> Result<()>;
    fn tell(&mut self) -> Result<usize>;
    fn write(&mut self, buf: &[u8]) -> Result<()>;

    fn reported_size(&mut self) -> Result<usize> {
        let current_pos = self.tell()?;
        self.seek(SeekFrom::End(0))?;
        let result = self.tell();
        self.seek(SeekFrom::Start(current_pos as u64))?;

        result
    }
    
    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0u8];
        self.read(&mut buf)?;

        Ok(buf[0])
    }
    fn read_u16(&mut self) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.read(&mut buf)?;

        let value = u16::from_ne_bytes(buf);
        Ok(adjust_endianness_u16(value))
    }
    fn read_u16_array(&mut self, buffer: &mut [u16]) -> Result<()> {
        for item in buffer.iter_mut() {
            *item = self.read_u16()?;
        }
        Ok(())
    }
    fn read_u32(&mut self) -> Result<u32> {
        let mut buf = [0u8; 4];
        self.read(&mut buf)?;

        let value = u32::from_ne_bytes(buf);
        Ok(adjust_endianness_u32(value))
    }
    fn read_f32(&mut self) -> Result<f32> {
        // read as a u32 in case magic changes values read upside down due to endianness.
        let uint_value = self.read_u32()?;

        // flip from u32 to f32
        unsafe { Ok(transmute::<u32, f32>(uint_value)) }
    }
    fn read_u64(&mut self) -> Result<u64> {
        let mut buf = [0u8; 8];
        self.read(&mut buf)?;

        let value = u64::from_ne_bytes(buf);
        Ok(adjust_endianness_u64(value))
    }
    fn read_s15f16(&mut self) -> Result<f64> {
        let fixed_point = unsafe { transmute::<u32, S15F16>(self.read_u32()?) };
        Ok(s15f16_to_f64(fixed_point))
    }
    fn read_xyz(&mut self) -> Result<CIEXYZ> {
        let x = self.read_s15f16()?;
        let y = self.read_s15f16()?;
        let z = self.read_s15f16()?;

        Ok(CIEXYZ { X: x, Y: y, Z: z })
    }

    fn write_u8(&mut self, value: u8) -> Result<()> {
        self.write(&[value])
    }
    fn write_u16(&mut self, value: u16) -> Result<()> {
        let value = adjust_endianness_u16(value);
        self.write(&value.to_ne_bytes())
    }
    fn write_u16_array(&mut self, buffer: &[u16]) -> Result<()> {
        for value in buffer.iter() {
            self.write_u16(*value)?;
        }
        Ok(())
    }
    fn write_u32(&mut self, value: u32) -> Result<()> {
        let value = adjust_endianness_u32(value);
        self.write(&value.to_ne_bytes())
    }
    fn write_f32(&mut self, value: f32) -> Result<()> {
        // flip from f32 to u32
        let uint_value = unsafe { transmute::<f32, u32>(value) };

        self.write_u32(uint_value)
    }
    fn write_u64(&mut self, value: u64) -> Result<()> {
        let value = adjust_endianness_u64(value);
        self.write(&value.to_ne_bytes())
    }
    fn write_s15f16(&mut self, value: f64) -> Result<()> {
        let fixed_point = f64_to_s15f16(value);
        self.write_u32(unsafe { transmute::<i32, u32>(fixed_point) })
    }
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
