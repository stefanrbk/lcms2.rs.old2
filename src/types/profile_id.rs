/// Profile ID as computed by MD5 algorithm
#[derive(Copy, Clone, Eq)]
pub union ProfileID {
    pub id8: [u8; 16],
    pub id16: [u16; 8],
    pub id32: [u32; 4],
}

impl std::fmt::Debug for ProfileID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut value = String::with_capacity(64);
        for i in 0..16 {
            unsafe {
                value.push_str(format!(" {:2x} ", self.id8[i]).as_str());
            }
        }

        f.debug_struct("ProfileID").field("value", &value).finish()
    }
}
impl PartialEq for ProfileID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.id8 == other.id8 }
    }
}
