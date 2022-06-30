use crate::conversions::pcs::xyY_to_XYZ;

use super::CIExyY;

#[allow(non_snake_case)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct CIEXYZ {
    pub X: f64,
    pub Y: f64,
    pub Z: f64,
}

impl From<CIExyY> for CIEXYZ {
    fn from(source: CIExyY) -> Self {
        xyY_to_XYZ(source)
    }
}
