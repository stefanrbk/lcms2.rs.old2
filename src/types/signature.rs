use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Signature(u32);

impl Signature {
    pub const fn new(value: &[u8; 4]) -> Signature {
        Self(u32::from_be_bytes(*value))
    }
}
impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let bytes = u32::to_be_bytes(self.0);
        let value: String = match std::str::from_utf8(&bytes) {
            Ok(i) => i.to_string(),
            Err(_) => {
                let mut line = String::with_capacity(16);
                for i in 0..4 {
                    line.push_str(format!(" {:2x} ", bytes[i]).as_str());
                }
                line
            }
        };

        f.debug_struct("Signature").field("value", &value).finish()
    }
}
impl Default for Signature {
    fn default() -> Self {
        Self::new(b"    ")
    }
}
impl From<Signature> for u32 {
    fn from(item: Signature) -> u32 {
        item.0
    }
}
impl From<u32> for Signature {
    fn from(item: u32) -> Self {
        Self(item)
    }
}
impl From<Signature> for [u8; 4] {
    fn from(value: Signature) -> Self {
        u32::to_be_bytes(value.0)
    }
}
impl From<&[u8; 4]> for Signature {
    fn from(value: &[u8; 4]) -> Self {
        Self(u32::from_be_bytes(*value))
    }
}
impl From<&[u8; 3]> for Signature {
    fn from(value: &[u8; 3]) -> Self {
        let mut result: [u8; 4] = [0x20; 4];
        result[..3].copy_from_slice(&*value);
        Self(u32::from_be_bytes(result))
    }
}
impl From<&[u8; 2]> for Signature {
    fn from(value: &[u8; 2]) -> Self {
        let mut result: [u8; 4] = [0x20; 4];
        result[..2].copy_from_slice(&*value);
        Self(u32::from_be_bytes(result))
    }
}
impl From<&[u8; 1]> for Signature {
    fn from(value: &[u8; 1]) -> Self {
        let mut result: [u8; 4] = [0x20; 4];
        result[..1].copy_from_slice(&*value);
        Self(u32::from_be_bytes(result))
    }
}
impl From<&[u8]> for Signature {
    fn from(value: &[u8]) -> Self {
        let len = value.len();
        let mut result: [u8; 4] = [0x20; 4];
        match len {
            i if i <= 0 => result = [0, 0, 0, 0],
            i if i == 1 => result[..1].copy_from_slice(&value[..1]),
            i if i == 2 => result[..2].copy_from_slice(&value[..2]),
            i if i == 3 => result[..3].copy_from_slice(&value[..3]),
            _ => result.copy_from_slice(&value[..4]),
        }
        Self::from(&result)
    }
}
impl From<Signature> for String {
    fn from(value: Signature) -> Self {
        match std::str::from_utf8(&u32::to_be_bytes(value.0)) {
            Err(_) => "    ".to_string(),
            Ok(i) => i.to_string(),
        }
    }
}
impl From<&str> for Signature {
    fn from(s: &str) -> Self {
        Self::from(s.as_bytes())
    }
}
