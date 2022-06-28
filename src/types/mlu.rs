pub struct MluEntry {
    language: [char; 2],
    country: [char; 2],

    byte_offset: u32,
    byte_len: u32,
}

impl MluEntry {
    pub const NO_LANGUAGE: [char; 2] = ['\0', '\0'];
    pub const NO_COUNTRY: [char; 2] = ['\0', '\0'];
}

pub struct Mlu {
    entries: Vec<MluEntry>,
    raw_chars: Vec<u16>,
}
// &mut Context must be passed in for all functions involving Mlu

fn str_to_char_2(str: impl Into<String>) -> [char; 2] {
    let str = str.into();
    let str = str.as_bytes();

    match str.len() {
        0 => ['\0', '\0'],
        1 => [str[0] as char, '\0'],
        _ => [str[0] as char, str[1] as char],
    }
}

fn str_from_char_2(value: [char; 2]) -> String {
    let mut str = String::with_capacity(2);

    str.push(value[0]);
    str.push(value[1]);

    str
}
