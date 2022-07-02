#[allow(non_snake_case)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct CIEXYZ {
    pub X: f64,
    pub Y: f64,
    pub Z: f64,
}
union CIEXYZBytes {
    pub value: CIEXYZ,
    pub bytes: [u8; 24],
}
impl CIEXYZ {
    pub(crate) fn to_bytes(&self) -> [u8; 24] {
        unsafe{
            CIEXYZBytes { value: *self }.bytes
        }
    }

    pub(crate) fn from_bytes(ptr: &[u8]) -> CIEXYZ {
        let len = ptr.len();
        let mut new_ptr = [0u8; 24];

        if len < 24 {
            new_ptr[0..len].copy_from_slice(ptr);
        } else {
            new_ptr.copy_from_slice(&ptr[0..24]);
        }

        unsafe {
            CIEXYZBytes { bytes: new_ptr }.value
        }
    }
}
