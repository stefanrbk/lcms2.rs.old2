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
