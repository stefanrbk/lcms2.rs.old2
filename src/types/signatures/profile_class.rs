use super::Signature;

pub const INPUT: Signature = Signature::new(b"scnr");
pub const DISPLAY: Signature = Signature::new(b"mntr");
pub const OUTPUT: Signature = Signature::new(b"prtr");
pub const LINK: Signature = Signature::new(b"link");
pub const ABSTRACT: Signature = Signature::new(b"abst");
pub const COLOR_SPACE: Signature = Signature::new(b"spac");
pub const NAMED_COLOR: Signature = Signature::new(b"nmcl");
