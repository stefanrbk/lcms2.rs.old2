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
    fn swap_endianness(header: ICCHeader) -> ICCHeader {
        let mut value = header.clone();

        value.size = adjust_endianness_u32(value.size);
        value.cmm_id = Signature::from(adjust_endianness_u32(value.cmm_id.into()));
        value.version = adjust_endianness_u32(value.version);
        value.device_class = Signature::from(adjust_endianness_u32(value.device_class.into()));
        value.color_space = Signature::from(adjust_endianness_u32(value.color_space.into()));
        value.pcs = Signature::from(adjust_endianness_u32(value.pcs.into()));
        value.date.year = adjust_endianness_u16(value.date.year);
        value.date.month = adjust_endianness_u16(value.date.month);
        value.date.day = adjust_endianness_u16(value.date.day);
        value.date.hours = adjust_endianness_u16(value.date.hours);
        value.date.minutes = adjust_endianness_u16(value.date.minutes);
        value.date.seconds = adjust_endianness_u16(value.date.seconds);
        value.magic = Signature::from(adjust_endianness_u32(value.magic.into()));
        value.platform = Signature::from(adjust_endianness_u32(value.platform.into()));
        value.flags = adjust_endianness_u32(value.flags);
        value.manufacturer = Signature::from(adjust_endianness_u32(value.manufacturer.into()));
        value.model = adjust_endianness_u32(value.model);
        value.attributes = adjust_endianness_u64(value.attributes);
        value.rendering_intent = adjust_endianness_u32(value.rendering_intent);
        value.illuminant.x = unsafe {
            transmute::<u32, i32>(adjust_endianness_u32(transmute::<i32, u32>(
                value.illuminant.x,
            )))
        };
        value.illuminant.y = unsafe {
            transmute::<u32, i32>(adjust_endianness_u32(transmute::<i32, u32>(
                value.illuminant.y,
            )))
        };
        value.illuminant.z = unsafe {
            transmute::<u32, i32>(adjust_endianness_u32(transmute::<i32, u32>(
                value.illuminant.z,
            )))
        };
        value.creator = Signature::from(adjust_endianness_u32(value.creator.into()));

        value
    }
    pub fn from_bytes(bytes: [u8; 128]) -> ICCHeader {
        let converter = ICCHeaderConverter { bytes };
        unsafe {
            Self::swap_endianness(converter.value)
        }
    }
    pub fn to_bytes(value: ICCHeader) -> [u8; 128] {
        let converter = ICCHeaderConverter { value: Self::swap_endianness(value) };
        unsafe {
            converter.bytes
        }
    }
}
