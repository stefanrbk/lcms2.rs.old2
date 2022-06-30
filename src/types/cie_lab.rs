#[allow(non_snake_case)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct CIELab {
    pub L: f64,
    pub a: f64,
    pub b: f64,
}
