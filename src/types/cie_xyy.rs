use crate::conversions::pcs::XYZ_to_xyY;

use super::CIEXYZ;

#[allow(non_snake_case)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct CIExyY {
    pub x: f64,
    pub y: f64,
    pub Y: f64,
}

impl From<CIEXYZ> for CIExyY {
    fn from(source: CIEXYZ) -> Self {
        XYZ_to_xyY(source)
    }
}
