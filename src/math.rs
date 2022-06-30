/// Fast floor conversion logic. Thanks to Sree Kotay and Stuart Nixon
/// note than this only works in the range ..-32767...+32767 because
/// mantissa is interpreted as 15.16 fixed point.
#[inline(always)]
pub fn quick_floor(val: f64) -> i32 {
    #[cfg(feature = "fast_floor")]
    {
        const DOUBLE_TO_FIX_MAGIC: f64 = 68719476736.0 * 1.5; // 2^36 * 1.5, (52-16=36) uses limited precision to floor
        union Temp {
            val: f64,
            halves: [i32; 2]
        }
        let temp = Temp {val: val + DOUBLE_TO_FIX_MAGIC};

        unsafe {
            return temp.halves[0] >> 16;
        }
    }
    #[cfg(not(feature = "fast_floor"))]
    {
        val.floor() as i32
    }
}

/// Fast floor restricted to 0..65536.0
#[inline(always)]
pub fn quick_floor_word(val: f64) -> u16 {
    u16::wrapping_add(quick_floor(val - 32767.0) as u16,32767)
}

/// Floor to word, taking care of saturation
#[inline(always)]
pub fn quick_saturate_word(val: f64) -> u16 {
    let val = val + 0.5;
    if val <= 0.0 {
        return 0;
    }
    if val >= 65535.0 {
        return 0xFFFF;
    }
    quick_floor_word(val)
}
