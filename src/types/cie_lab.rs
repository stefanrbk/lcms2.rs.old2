#![allow(non_snake_case)]

use crate::conversions::pcs::Lab_to_XYZ;

use super::CIEXYZ;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct CIELab {
    pub L: f64,
    pub a: f64,
    pub b: f64,
}

impl CIELab {
    #[inline(always)]
    pub fn to_XYZ(self, whitepoint: Option<CIEXYZ>) -> CIEXYZ {
        Lab_to_XYZ(whitepoint, self)
    }
}
