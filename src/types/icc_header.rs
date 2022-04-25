use std::intrinsics::transmute;

use crate::io::*;

use super::*;

/// Profile Header -- 32-bit aligned
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct ICCHeader {
    /// Profile size in bytes
    pub size: u32,
    /// CMM for this profile
    pub cmm_id: Signature,
    /// Format version number
    pub version: u32,
    /// Type of profile
    pub device_class: Signature,
    /// Color space of data
    pub color_space: Signature,
    /// PCS, XYZ or LAB only
    pub pcs: Signature,
    /// Date profile was created
    pub date: DateTimeNumber,
    /// Magic Number to identity an ICC profile
    pub magic: Signature,
    /// Primary platform
    pub platform: Signature,
    /// Various bit settings
    pub flags: u32,
    /// Device manufacturer
    pub manufacturer: Signature,
    /// Device model Number
    pub model: u32,
    /// Device attributes
    pub attributes: u64,
    /// Rendering intent
    pub rendering_intent: u32,
    /// Profile illuminant
    pub illuminant: EncodedXYZNumber,
    /// Profile creator
    pub creator: Signature,
    /// Profile ID using MD5
    pub profile_id: ProfileID,
    /// Reserved for future use
    pub reserved: [u8; 28],
}

pub(crate) union ICCHeaderConverter {
    value: ICCHeader,
    bytes: [u8; 128],
}

impl ICCHeaderConverter {
    fn swap_endianness(header: &ICCHeader) -> ICCHeader {
        let mut header = header.clone();

        header.size = adjust_endianness_u32(header.size);
        header.cmm_id = Signature::from(adjust_endianness_u32(header.cmm_id.into()));
        header.version = adjust_endianness_u32(header.version);
        header.device_class = Signature::from(adjust_endianness_u32(header.device_class.into()));
        header.color_space = Signature::from(adjust_endianness_u32(header.color_space.into()));
        header.pcs = Signature::from(adjust_endianness_u32(header.pcs.into()));
        header.date.year = adjust_endianness_u16(header.date.year);
        header.date.month = adjust_endianness_u16(header.date.month);
        header.date.day = adjust_endianness_u16(header.date.day);
        header.date.hours = adjust_endianness_u16(header.date.hours);
        header.date.minutes = adjust_endianness_u16(header.date.minutes);
        header.date.seconds = adjust_endianness_u16(header.date.seconds);
        header.magic = Signature::from(adjust_endianness_u32(header.magic.into()));
        header.platform = Signature::from(adjust_endianness_u32(header.platform.into()));
        header.flags = adjust_endianness_u32(header.flags);
        header.manufacturer = Signature::from(adjust_endianness_u32(header.manufacturer.into()));
        header.model = adjust_endianness_u32(header.model);
        header.attributes = adjust_endianness_u64(header.attributes);
        header.rendering_intent = adjust_endianness_u32(header.rendering_intent);
        header.illuminant.x = unsafe {
            transmute::<u32, i32>(adjust_endianness_u32(transmute::<i32, u32>(
                header.illuminant.x,
            )))
        };
        header.illuminant.y = unsafe {
            transmute::<u32, i32>(adjust_endianness_u32(transmute::<i32, u32>(
                header.illuminant.y,
            )))
        };
        header.illuminant.z = unsafe {
            transmute::<u32, i32>(adjust_endianness_u32(transmute::<i32, u32>(
                header.illuminant.z,
            )))
        };
        header.creator = Signature::from(adjust_endianness_u32(header.creator.into()));

        header
    }
    pub fn from_bytes(bytes: [u8; 128]) -> ICCHeader {
        let converter = ICCHeaderConverter { bytes };
        unsafe {
            Self::swap_endianness(&converter.value)
        }
    }
    pub fn to_bytes(value: ICCHeader) -> [u8; 128] {
        let converter = ICCHeaderConverter { value: Self::swap_endianness(&value) };
        unsafe {
            converter.bytes
        }
    }
}
