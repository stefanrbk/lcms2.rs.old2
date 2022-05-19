use super::Signature;

pub mod tag;
pub mod tag_type;

pub const MAGIC_NUMBER: Signature = Signature::new(b"acsp");
pub const LCMS_SIGNATURE: Signature = Signature::new(b"lcms");

pub mod color_space;
pub mod platform;
pub mod profile_class;
pub mod technology;
pub mod plugin_type;

// Reference gamut
pub const PERCEPTUAL_REFERENCE_MEDIUM_GAMUT: Signature = Signature::new(b"prmg");

// For SigColorimetricIntentImageStateTag
pub const SCENE_COLORIMETRY_ESTIMATES: Signature = Signature::new(b"scoe");
pub const SCENE_APPEARANCE_ESTIMATES: Signature = Signature::new(b"sape");
pub const FOCAL_PLANE_COLORIMETRY_ESTIMATES: Signature = Signature::new(b"fpce");
pub const REFLECTION_HARDCOPY_ORIGINAL_COLORIMETRY: Signature = Signature::new(b"rhoc");
pub const REFLECTION_PRINT_OUTPUT_COLORIMETRY: Signature = Signature::new(b"rpoc");

pub mod stage;
pub mod curve_segment {
    use super::Signature;

    pub const FORMULA: Signature = Signature::new(b"parf");
    pub const SAMPLED: Signature = Signature::new(b"samf");
    pub const SEGMENTED: Signature = Signature::new(b"curf");
}
pub const STATUS_A: Signature = Signature::new(b"StaA");
pub const STATUS_E: Signature = Signature::new(b"StaE");
pub const STATUS_I: Signature = Signature::new(b"StaI");
pub const STATUS_T: Signature = Signature::new(b"StaT");
pub const STATUS_M: Signature = Signature::new(b"StaM");
pub const DN: Signature = Signature::new(b"DN  ");
pub const DNP: Signature = Signature::new(b"DN P");
pub const DNN: Signature = Signature::new(b"DNN ");
pub const DNNP: Signature = Signature::new(b"DNNP");
