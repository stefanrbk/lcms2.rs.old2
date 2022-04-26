use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Signature(u32);

impl Signature {
    pub const fn new(value: &[u8; 4]) -> Signature {
        Self(u32::from_be_bytes(*value))
    }
}
#[cfg_attr(tarpaulin, skip)]
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
            i if i <= 0 => result = result,
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

#[cfg(test)]
mod test {
    use crate::types::signatures::color_space;

    use super::*;

    #[test]
    fn test_signature_gray_is_correct() {
        let expected = u32::from_be_bytes(*b"GRAY");
        let actual = Signature::new(b"GRAY").0;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_signature_from_str_is_correct() {
        let expected = color_space::GRAY;
        let actual = Signature::from("GRAY");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_signature_default_is_4_spaces() {
        let expected = Signature::new(b"    ");
        let actual: Signature = Default::default();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_signature_from_array_of_4_is_correct() {
        let expected = color_space::GRAY;
        let actual = Signature::from(b"GRAY");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_signature_from_array_of_3_is_correct() {
        let expected = color_space::RGB;
        let actual = Signature::from(b"RGB");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_signature_from_array_of_2_is_correct() {
        let expected = Signature::new(b"xy  ");
        let actual = Signature::from(b"xy");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_signature_from_array_of_1_is_correct() {
        let expected = Signature::new(b"a   ");
        let actual = Signature::from(b"a");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_signature_from_slice_of_more_than_4_is_correct() {
        let expected = Signature::new(b"abcd");
        let actual = Signature::from(b"abcdefg".as_slice());

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_signature_from_slice_of_4_is_correct() {
        let expected = color_space::GRAY;
        let actual = Signature::from(b"GRAY".as_slice());

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_signature_from_slice_of_3_is_correct() {
        let expected = color_space::RGB;
        let actual = Signature::from(b"RGB".as_slice());

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_signature_from_slice_of_2_is_correct() {
        let expected = Signature::new(b"xy  ");
        let actual = Signature::from(b"xy".as_slice());

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_signature_from_slice_of_1_is_correct() {
        let expected = Signature::new(b"a   ");
        let actual = Signature::from(b"a".as_slice());

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_signature_from_slice_of_0_is_default() {
        let expected: Signature = Default::default();
        let actual = Signature::from(&b"a"[0..0]);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_signature_from_u32_is_correct() {
        let expected = color_space::GRAY;
        let actual = Signature::from(u32::from_be_bytes(*b"GRAY"));

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_signature_to_u32_is_correct() {
        let expected = u32::from_be_bytes(*b"GRAY");
        let actual: u32 = color_space::GRAY.into();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_signature_to_array_of_4_is_correct() {
        let expected = *b"GRAY";
        let actual: [u8; 4] = color_space::GRAY.into();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_signature_to_string_is_correct() {
        let expected = "GRAY".to_string();
        let actual: String = color_space::GRAY.into();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_signature_to_string_with_invalid_data_is_default() {
        let expected: String = Signature::default().into();
        let actual: String = Signature::new(&[0, 159, 146, 150]).into();

        assert_eq!(expected, actual);
    }
}
