use super::*;
use crate::state::*;
use crate::types::*;
use crate::*;

type Result<T> = std::io::Result<T>;

pub struct IOHandler {
    stream: Box<dyn Stream>,
    context: Context,
}

impl IOHandler {
    pub fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0u8];
        self.stream.read_exact(&mut buf)?;

        Ok(buf[0])
    }
    pub fn read_u16(&mut self) -> Result<u16> {
        let mut buf = [0u8; 2];
        self.stream.read_exact(&mut buf)?;

        let value = u16::from_ne_bytes(buf);
        Ok(adjust_endianness_u16(value))
    }
    pub fn read_u16_array(&mut self, buffer: &mut [u16]) -> Result<()> {
        for item in buffer.iter_mut() {
            *item = self.read_u16()?;
        }
        Ok(())
    }
    pub fn read_u32(&mut self) -> Result<u32> {
        let mut buf = [0u8; 4];
        self.stream.read_exact(&mut buf)?;

        let value = u32::from_ne_bytes(buf);
        Ok(adjust_endianness_u32(value))
    }
    pub fn read_f32(&mut self) -> Result<f32> {
        // read as a u32 in case magic changes values read upside down due to endianness.
        let uint_value = self.read_u32()?;

        // flip from u32 to f32
        Ok(f32::from_ne_bytes(uint_value.to_ne_bytes()))
    }
    pub fn read_u64(&mut self) -> Result<u64> {
        let mut buf = [0u8; 8];
        self.stream.read_exact(&mut buf)?;

        let value = u64::from_ne_bytes(buf);
        Ok(adjust_endianness_u64(value))
    }
    pub fn read_s15f16(&mut self) -> Result<f64> {
        let fixed_point = S15F16::from_ne_bytes(self.read_u32()?.to_ne_bytes());
        Ok(s15f16_to_f64(fixed_point))
    }
    pub fn read_xyz(&mut self) -> (f32, f32, f32) {
        (0.0, 0.0, 0.0)
    }

    pub fn write_u8(&mut self, value: u8) {}
    pub fn write_u16(&mut self, value: u16) {}
    pub fn write_u32(&mut self, value: u32) {}
    pub fn write_f32(&mut self, value: f32) {}
    pub fn write_u64(&mut self, value: u64) {}
    pub fn write_s15f16(&mut self, value: f64) {}
    pub fn write_xyz(&mut self, value: (f32, f32, f32)) {}
    pub fn write_u16_array(&mut self, buffer: &[u16]) {}
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