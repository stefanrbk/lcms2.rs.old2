//! Helper functions to convert from little endian into native endian and vice versa.

/// Convert a `[u8; 2]` array to/from little endian.
pub fn adjust_endianness_16(value: [u8; 2]) -> [u8; 2] {
    let value = u16::from_le_bytes(value);
    value.to_ne_bytes()
}

/// Convert a `[u8; 4]` array to/from little endian.
pub fn adjust_endianness_32(value: [u8; 4]) -> [u8; 4] {
    let value = u32::from_le_bytes(value);
    value.to_ne_bytes()
}

/// Convert a `[u8; 8]` array to/from little endian.
pub fn adjust_endianness_64(value: [u8; 8]) -> [u8; 8] {
    let value = u64::from_le_bytes(value);
    value.to_ne_bytes()
}

/// Convert a [`u16`] to/from little endian.
pub fn adjust_endianness_u16(value: u16) -> u16 {
    value.to_le()
}

/// Convert a [`u32`] to/from little endian.
pub fn adjust_endianness_u32(value: u32) -> u32 {
    value.to_le()
}

/// Convert a [`u64`] to/from little endian.
pub fn adjust_endianness_u64(value: u64) -> u64 {
    value.to_le()
}
