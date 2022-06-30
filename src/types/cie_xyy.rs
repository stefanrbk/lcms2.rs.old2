#![allow(non_snake_case)]

use crate::conversions::pcs::XYZ_to_xyY;

use super::CIEXYZ;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct CIExyY {
    pub x: f64,
    pub y: f64,
    pub Y: f64,
}

impl CIExyY {
    #[inline(always)]
    pub fn to_XYZ(self) -> CIEXYZ {
        self.into()
    }
}

impl From<CIEXYZ> for CIExyY {
    fn from(source: CIEXYZ) -> Self {
        XYZ_to_xyY(source)
    }
}
