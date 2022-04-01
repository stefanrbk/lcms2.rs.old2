use crate::S15F16;

/// ICC XYZ
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct EncodedXYZNumber {
    pub x: S15F16,
    pub y: S15F16,
    pub z: S15F16,
}
