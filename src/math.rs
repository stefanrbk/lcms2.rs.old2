use crate::U8F8;

#[inline]
pub fn u8f8_to_f64(fixed8: U8F8) -> f64 {
    let lsb = (fixed8 & 0xFF) as u8;
    let msb = ((fixed8 >> 8) & 0xFF) as u8;
    
    msb as f64 + (lsb as f64 / 256.0)
}
