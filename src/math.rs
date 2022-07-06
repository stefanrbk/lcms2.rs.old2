use crate::{U8F8, S15F16};

#[inline]
pub fn quick_floor(val: f64) -> i32 {
    #[cfg(feature = "fast_floor")]
    {
        const MAGIC: f64 = 68719476736.0 * 1.5;  // 2^36 * 1.5, (52-16=36) uses limited precision to floor
        union Temp {
            val: f64,
            halves: [i32; 2]
        }

        let temp = Temp{ val: val + MAGIC };
        unsafe {
            temp.halves[0] >> 16
        }
    }
    #[cfg(not(feature = "fast_floor"))]
    {
        val.floor() as i32
    }
}

#[inline]
pub fn quick_floor_word(d: f64) -> u16 {
    u16::wrapping_add(quick_floor(d - 32767.0) as u16, 32767)
}

#[inline]
pub fn quick_saturate_word(d: f64) -> u16 {
    let d = d + 0.5;
    if d <= 0.0 { return 0 }
    if d >= 65535.0 { return 0xFFFF }

    quick_floor_word(d)
}

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

#[inline]
pub fn u16_slice_to_f32_slice(r#in: &[u16], out: &mut [f32]) {
    let len = out.len();
    for i in 0..len {
        out[i] = r#in[i] as f32 / 65535.0f32;
    }
}

#[inline]
pub fn f32_slice_to_u16_slice(r#in: &[f32], out: &mut [u16]) {
    let len = out.len();
    for i in 0..len {
        out[i] = quick_saturate_word(r#in[i] as f64 * 65535.0);
    }
}
