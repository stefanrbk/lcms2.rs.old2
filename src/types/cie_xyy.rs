#[allow(non_snake_case)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct CIExyY {
    pub x: f64,
    pub y: f64,
    pub Y: f64,
}
