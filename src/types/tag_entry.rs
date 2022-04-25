use crate::io::adjust_endianness_u32;

use super::Signature;

/// A tag entry in dictionary
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct TagEntry {
    /// The tag signature
    pub sig: Signature,
    /// Start of tag
    pub offset: u32,
    /// Size in bytes
    pub size: u32,
}

pub(crate) union TagEntryConverter {
    value: TagEntry,
    bytes: [u8; 12],
}

impl TagEntryConverter {
    fn swap_endianness(entry: &TagEntry) -> TagEntry {
        let mut entry = entry.clone();

        entry.sig = Signature::from(adjust_endianness_u32(entry.sig.into()));
        entry.offset = adjust_endianness_u32(entry.offset);
        entry.size = adjust_endianness_u32(entry.size);

        entry
    }
    pub fn from_bytes(bytes: [u8; 12]) -> TagEntry {
        let converter = TagEntryConverter { bytes };
        unsafe {
            Self::swap_endianness(&converter.value)
        }
    }
    pub fn to_bytes(value: TagEntry) -> [u8; 12] {
        let converter = TagEntryConverter { value: Self::swap_endianness(&value) };
        unsafe {
            converter.bytes
        }
    }
}
