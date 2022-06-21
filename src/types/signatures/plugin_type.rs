use super::Signature;

pub const MAGIC: Signature = Signature::new(b"acpp");
pub const TRANSFORM: Signature = Signature::new(b"xfmH");
pub const INTERPOLATION: Signature = Signature::new(b"inpH");
pub const TAG_TYPE: Signature = Signature::new(b"typH");
pub const TAG: Signature = Signature::new(b"tagH");
pub const FORMATTERS: Signature = Signature::new(b"frmH");
pub const RENDERING_INTENT: Signature = Signature::new(b"intH");
pub const PARAMETRIC_CURVE: Signature = Signature::new(b"parH");
pub const MULTI_PROCESS_ELEMENT: Signature = Signature::new(b"mpeH");
pub const OPTIMIZATION: Signature = Signature::new(b"optH");
