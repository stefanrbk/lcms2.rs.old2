#![allow(non_snake_case)]

use crate::conversions::pcs::{xyY_to_XYZ, XYZ_to_Lab};

use super::{CIExyY, CIELab};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct CIEXYZ {
    pub X: f64,
    pub Y: f64,
    pub Z: f64,
}

impl CIEXYZ {
    #[inline(always)]
    pub fn to_xyY(self) -> CIExyY {
        self.into()
    }
    #[inline(always)]
    pub fn to_Lab(self, whitepoint: Option<CIEXYZ>) -> CIELab {
        XYZ_to_Lab(whitepoint, self)
    }
}

impl From<CIExyY> for CIEXYZ {
    fn from(source: CIExyY) -> Self {
        xyY_to_XYZ(source)
    }
}
