#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::conversions::pcs::{Lab_to_XYZ, Lab_encoded_to_float, float_to_Lab_encoded};

use super::CIEXYZ;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct CIELab {
    pub L: f64,
    pub a: f64,
    pub b: f64,
}

impl CIELab {
    pub const MIN_ENCODEABLE_ab2: f64 = -128.0;
    pub const MAX_ENCODEABLE_ab2: f64 = (65535.0/256.0) - 128.0;
    pub const MIN_ENCODEABLE_ab4: f64 = -128.0;
    pub const MAX_ENCODEABLE_ab4: f64 = 127.0;

    #[inline(always)]
    pub fn to_XYZ(self, whitepoint: Option<CIEXYZ>) -> CIEXYZ {
        Lab_to_XYZ(whitepoint, self)
    }
}

impl From<[u16; 3]> for CIELab {
    fn from(values: [u16; 3]) -> Self {
        Lab_encoded_to_float(values)
    }
}

impl From<CIELab> for [u16; 3] {
    fn from(value: CIELab) -> Self {
        float_to_Lab_encoded(value)
    }
}
