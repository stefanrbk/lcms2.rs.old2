use super::Signature;

pub const A_TO_B0: Signature = Signature::new(b"A2B0");
pub const A_TO_B1: Signature = Signature::new(b"A2B1");
pub const A_TO_B2: Signature = Signature::new(b"A2B2");
pub const BLUE_COLORANT: Signature = Signature::new(b"bXYZ");
pub const BLUE_MATRIX_COLUMN: Signature = Signature::new(b"bXYZ");
pub const BLUE_TRC: Signature = Signature::new(b"bTRC");
pub const B_TO_A0: Signature = Signature::new(b"B2A0");
pub const B_TO_A1: Signature = Signature::new(b"B2A1");
pub const B_TO_A2: Signature = Signature::new(b"B2A2");
pub const CALIBRATION_DATE_TIME: Signature = Signature::new(b"calt");
pub const CHAR_TARGET: Signature = Signature::new(b"targ");
pub const CHROMATIC_ADAPTATION: Signature = Signature::new(b"chad");
pub const CHROMATICITY: Signature = Signature::new(b"chrm");
pub const COLORANT_ORDER: Signature = Signature::new(b"clro");
pub const COLORANT_TABLE: Signature = Signature::new(b"clrt");
pub const COLORANT_TABLE_OUT: Signature = Signature::new(b"clot");
pub const COLORIMETRIC_INTENT_IMAGE_STATE: Signature = Signature::new(b"ciis");
pub const COPYRIGHT: Signature = Signature::new(b"cprt");
pub const CRD_INFO: Signature = Signature::new(b"crdi");
pub const DATA: Signature = Signature::new(b"data");
pub const DATE_TIME: Signature = Signature::new(b"dtim");
pub const DEVICE_MFG_DESC: Signature = Signature::new(b"dmnd");
pub const DEVICE_MODEL_DESC: Signature = Signature::new(b"dmdd");
pub const DEVICE_SETTINGS: Signature = Signature::new(b"devs");
pub const D_TO_B0: Signature = Signature::new(b"D2B0");
pub const D_TO_B1: Signature = Signature::new(b"D2B1");
pub const D_TO_B2: Signature = Signature::new(b"D2B2");
pub const D_TO_B3: Signature = Signature::new(b"D2B3");
pub const B_TO_D0: Signature = Signature::new(b"B2D0");
pub const B_TO_D1: Signature = Signature::new(b"B2D1");
pub const B_TO_D2: Signature = Signature::new(b"B2D2");
pub const B_TO_D3: Signature = Signature::new(b"B2D3");
pub const GAMUT: Signature = Signature::new(b"gamt");
pub const GRAY_TRC: Signature = Signature::new(b"kTRC");
pub const GREEN_COLORANT: Signature = Signature::new(b"gXYZ");
pub const GREEN_MATRIX_COLUMN: Signature = Signature::new(b"gXYZ");
pub const GREEN_TRC: Signature = Signature::new(b"gTRC");
pub const LUMINANCE: Signature = Signature::new(b"lumi");
pub const MEASUREMENT: Signature = Signature::new(b"meas");
pub const MEDIA_BLACK_POINT: Signature = Signature::new(b"bkpt");
pub const MEDIA_WHITE_POINT: Signature = Signature::new(b"wtpt");
pub const NAMED_COLOR: Signature = Signature::new(b"ncol");
pub const NAMED_COLOR2: Signature = Signature::new(b"ncl2");
pub const OUTPUT_RESPONSE: Signature = Signature::new(b"resp");
pub const PERCEPTUAL_RENDERING_INTENT_GAMUT: Signature = Signature::new(b"rig0");
pub const PREVIEW0: Signature = Signature::new(b"pre0");
pub const PREVIEW1: Signature = Signature::new(b"pre1");
pub const PREVIEW2: Signature = Signature::new(b"pre2");
pub const PROFILE_DESCRIPTION: Signature = Signature::new(b"desc");
pub const PROFILE_DESCRIPTION_ML: Signature = Signature::new(b"dscm");
pub const PROFILE_SEQUENCE_DESC: Signature = Signature::new(b"pseq");
pub const PROFILE_SEQUENCE_ID: Signature = Signature::new(b"psid");
pub const PS2_CRD0: Signature = Signature::new(b"psd0");
pub const PS2_CRD1: Signature = Signature::new(b"psd1");
pub const PS2_CRD2: Signature = Signature::new(b"psd2");
pub const PS2_CRD3: Signature = Signature::new(b"psd3");
pub const PS2_CSA: Signature = Signature::new(b"ps2s");
pub const PS2_RENDERING_INTENT: Signature = Signature::new(b"ps2i");
pub const RED_COLORANT: Signature = Signature::new(b"rXYZ");
pub const RED_MATRIX_COLUMN: Signature = Signature::new(b"rXYZ");
pub const RED_TRC: Signature = Signature::new(b"rTRC");
pub const SATURATION_RENDERING_INTENT_GAMUT: Signature = Signature::new(b"rig2");
pub const SCREENING_DESC: Signature = Signature::new(b"scrd");
pub const SCREENING: Signature = Signature::new(b"scrn");
pub const TECHNOLOGY: Signature = Signature::new(b"tech");
pub const UCR_BG: Signature = Signature::new(b"bfd ");
pub const VIEWING_COND_DESC: Signature = Signature::new(b"vued");
pub const VIEWING_CONDITIONS: Signature = Signature::new(b"view");
pub const VCGT: Signature = Signature::new(b"vcgt");
pub const META: Signature = Signature::new(b"meta");
pub const ARGYLL_ARTS: Signature = Signature::new(b"arts");
