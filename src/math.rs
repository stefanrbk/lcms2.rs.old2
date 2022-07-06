use crate::{U8F8, S15F16};

#[inline]
pub fn u8f8_to_f64(fixed8: U8F8) -> f64 {
    let lsb = (fixed8 & 0xFF) as u8;
    let msb = ((fixed8 >> 8) & 0xFF) as u8;
    
    msb as f64 + (lsb as f64 / 256.0)
}

#[inline]
pub fn f64_to_u8f8(val: f64) -> U8F8 {
    let gamma_fixed_32 = f64_to_s15f16(val);
    ((gamma_fixed_32 >> 8) & 0xFFFF) as U8F8
}

#[inline]
pub fn s15f16_to_f64(fix32: S15F16) -> f64 {
    let sign = if fix32 < 0 { -1.0 } else { 1.0 };
    let fix32 = fix32.abs();

    let whole = ((fix32 >> 16) & 0xFFFF) as u16;
    let frac = (fix32 & 0xFFFF) as u16;

    let mid = frac as f64 / 65536.0;
    let floater = whole as f64 + mid;
    
    sign * floater
}

#[inline]
pub fn f64_to_s15f16(val: f64) -> S15F16 {
    (val * 65536.0 + 0.5) as S15F16
}
