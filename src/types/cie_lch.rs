use crate::conversions::pcs::Lab_to_LCh;

use super::CIELab;

#[allow(non_snake_case)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct CIELCh {
    pub L: f64,
    pub C: f64,
    pub h: f64,
}

impl From<CIELab> for CIELCh {
    fn from(lab: CIELab) -> Self {
        Lab_to_LCh(lab)
    }
}
