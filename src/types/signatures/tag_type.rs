//! Constants representing ICC type definitions
use super::Signature;

/// Chromaticity ICC type (`chrm`)
pub const CHROMATICITY: Signature = Signature::new(b"chrm");
/// Colorant order ICC type (`clro`)
pub const COLORANT_ORDER: Signature = Signature::new(b"clro");
/// Colorant table ICC type (`clrt`)
pub const COLORANT_TABLE: Signature = Signature::new(b"clrt");
/// Color rendering dictionary info ICC type (`crdi`)
pub const CRD_INFO: Signature = Signature::new(b"crdi");
/// Curve ICC type (`curv`)
pub const CURVE: Signature = Signature::new(b"curv");
/// Data ICC type (`data`)
pub const DATA: Signature = Signature::new(b"data");
/// Dictionary ICC type (`dict`)
pub const DICT: Signature = Signature::new(b"dict");
/// Date/Time ICC type (`dtim`)
pub const DATE_TIME: Signature = Signature::new(b"dtim");
/// Device settings ICC type (`devs`)
pub const DEVICE_SETTINGS: Signature = Signature::new(b"devs");
/// Lookup table with 16 bit precision ICC type (`mft2`)
pub const LUT16: Signature = Signature::new(b"mft2");
/// Lookup table with 8 bit precision ICC type (`mft1`)
pub const LUT8: Signature = Signature::new(b"mft1");
/// Lookup table for "Device A to Device B" color transform ICC type (`mAB `)
pub const LUTA_TO_B: Signature = Signature::new(b"mAB ");
/// Lookup table for "Device B to Device A" color transform ICC type (`mBA `)
pub const LUTB_TO_A: Signature = Signature::new(b"mBA ");
/// Measurement ICC type (`meas`)
pub const MEASUREMENT: Signature = Signature::new(b"meas");
/// Multi-localized unicode string ICC type (`mluc`)
pub const MULTI_LOCALIZED_UNICODE: Signature = Signature::new(b"mluc");
/// Multi-process element ICC type (`mpet`)
pub const MULTI_PROCESS_ELEMENT: Signature = Signature::new(b"mpet");
/// Named color ICC type (`ncol`)
#[deprecated]
pub const NAMED_COLOR: Signature = Signature::new(b"ncol");
/// Named color ICC type (`ncl2`)
pub const NAMED_COLOR2: Signature = Signature::new(b"ncl2");
/// Parametric curve ICC type (`para`)
pub const PARAMETRIC_CURVE: Signature = Signature::new(b"para");
/// Profile sequence description ICC type (`pseq`)
pub const PROFILE_SEQUENCE_DESC: Signature = Signature::new(b"pseq");
/// Profile sequence ID ICC type (`psid`)
pub const PROFILE_SEQUENCE_ID: Signature = Signature::new(b"psid");
/// Response curve set 16 bit precision ICC type (`rcs2`)
pub const RESPONSE_CURVE_SET16: Signature = Signature::new(b"rcs2");
/// [`S15F16`] array ICC type (`sf32`)
/// [`S15F16`]: lcms2::S15F16
pub const S15_FIXED16_ARRAY: Signature = Signature::new(b"sf32");
/// Screening ICC type (`scrn`)
pub const SCREENING: Signature = Signature::new(b"scrn");
/// Signature ICC type (`sig `)
pub const SIGNATURE: Signature = Signature::new(b"sig ");
/// Text ICC type (`text`)
pub const TEXT: Signature = Signature::new(b"text");
/// Text description ICC type (`desc`)
pub const TEXT_DESCRIPTION: Signature = Signature::new(b"desc");
/// [`U16F16`] array ICC type (`uf32`)
/// [`U16F16`]: lcms2::U16F16
pub const U16_FIXED16_ARRAY: Signature = Signature::new(b"uf32");
/// Under color removal and black generation ICC type (`bfd `)
pub const UCR_BG: Signature = Signature::new(b"bfd ");
/// [`u16`] array ICC type (`ui16`)
pub const UINT16_ARRAY: Signature = Signature::new(b"ui16");
/// [`u32`] array ICC type (`ui32`)
pub const UINT32_ARRAY: Signature = Signature::new(b"ui32");
/// [`u64`] array ICC type (`ui64`)
pub const UINT64_ARRAY: Signature = Signature::new(b"ui64");
/// [`u8`] array ICC type (`ui98`)
pub const UINT8_ARRAY: Signature = Signature::new(b"ui08");
/// Video card gamma table ICC type (`vcgt`)
pub const VCGT: Signature = Signature::new(b"vcgt");
/// Viewing condition ICC type (`view`)
pub const VIEWING_CONDITIONS: Signature = Signature::new(b"view");
/// XYZ number ICC type (`XYZ `)
pub const XYZ: Signature = Signature::new(b"XYZ ");
