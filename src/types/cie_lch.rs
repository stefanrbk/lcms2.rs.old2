#[allow(non_snake_case)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct CIELCh {
    pub L: f64,
    pub C: f64,
    pub h: f64,
}
