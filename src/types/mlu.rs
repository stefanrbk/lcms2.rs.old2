use std::{error::Error, fmt::Display};

pub struct MluEntry {
    language: [char; 2],
    country: [char; 2],

    offset: usize,
    len: usize,
}

impl MluEntry {
    pub const NO_LANGUAGE: [char; 2] = ['\0', '\0'];
    pub const NO_COUNTRY: [char; 2] = ['\0', '\0'];
}

pub struct Mlu {
    entries: Vec<MluEntry>,
    raw_chars: Vec<u16>,
}

#[derive(Debug)]
pub struct MluDuplicateError([char; 2], [char; 2]);

impl Display for MluDuplicateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MLU entry for \"{}/{}\" already exists!", str_from_char_2(self.0), str_from_char_2(self.1))
    }
}
impl Error for MluDuplicateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}
impl Mlu {
    pub fn search_entry(&self, lang: [char; 2], cntry: [char; 2]) -> Option<&MluEntry> {
        for entry in self.entries.iter() {
            if entry.language == lang && entry.country == cntry {
                return Some(entry);
            }
        }
        None
    }
    pub fn search_entry_mut(&mut self, lang: [char; 2], cntry: [char; 2]) -> Option<&mut MluEntry> {
        for entry in self.entries.iter_mut() {
            if entry.language == lang && entry.country == cntry {
                return Some(entry);
            }
        }
        None
    }
    pub fn add_utf8(&mut self, lang: [char; 2], cntry: [char; 2], str: impl Into<String>) -> Result<(), MluDuplicateError> {
        let str: String = str.into();

        if self.search_entry(lang, cntry).is_some() {
            return Err(MluDuplicateError(lang, cntry));
        }

        let offset = self.raw_chars.len();
        let array = str.encode_utf16().collect::<Vec<u16>>();

        let len = array.len();
        if len == 0 {
            self.raw_chars.push(0);
            self.entries.push(MluEntry { language: lang, country: cntry, offset, len: 1 });

        } else {
            self.raw_chars.extend_from_slice(array.as_slice());
            self.entries.push(MluEntry { language: lang, country: cntry, offset, len });
        }

        Ok(())
    }
    pub fn add_utf16(&mut self, lang: [char; 2], cntry: [char; 2], str: &[u16]) -> Result<(), MluDuplicateError> {
        if self.search_entry(lang, cntry).is_some() {
            return Err(MluDuplicateError(lang, cntry));
        }

        let offset = self.raw_chars.len();
        let array = str;

        let len = array.len();
        if len == 0 {
            self.raw_chars.push(0);
            self.entries.push(MluEntry { language: lang, country: cntry, offset, len: 1 });

        } else {
            self.raw_chars.extend_from_slice(array);
            self.entries.push(MluEntry { language: lang, country: cntry, offset, len });
        }

        Ok(())
    }
    fn _get_utf16(&self, lang: [char; 2], cntry: [char; 2]) -> (&[u16], [char; 2], [char; 2]) {
        let mut best = -1;

        for i in 0..self.entries.len() {
            let v = &self.entries[i];
            if v.language == lang {
                if best == -1 {
                    best = i as i32;
                }

                if v.country == cntry {
                    return (&self.raw_chars.as_slice()[v.offset..(v.offset + v.len)], v.language, v.country);
                }
            }
        }

        // No string found. Return the first one
        if best == -1 {
            best = 0;
        }
        if self.entries.len() == 0 {
            return (&[], MluEntry::NO_LANGUAGE, MluEntry::NO_COUNTRY);
        }
        let v = &self.entries[best as usize];
        
        (&self.raw_chars.as_slice()[v.offset..(v.offset + v.len)], v.language, v.country)
    }
    pub fn get_utf8(&self, lang: impl Into<String>, cntry: impl Into<String>) -> String {
        let lang = str_to_char_2(lang);
        let cntry = str_to_char_2(cntry);

        let (utf16, _, _) = self._get_utf16(lang, cntry);

        match String::from_utf16(utf16) {
            Ok(result) => result,
            Err(_) => String::from(""),
        }
    }
    pub fn get_utf16(&self, lang: impl Into<String>, cntry: impl Into<String>) -> &[u16] {
        let lang = str_to_char_2(lang);
        let cntry = str_to_char_2(cntry);

        let (utf16, _, _) = self._get_utf16(lang, cntry);

        match String::from_utf16(utf16) {
            Ok(_) => utf16,
            Err(_) => &[],
        }
    }
    pub fn get_translation(&self, lang: impl Into<String>, cntry: impl Into<String>) -> (String, String) {
        let lang = str_to_char_2(lang);
        let cntry = str_to_char_2(cntry);

        let (_, lang, cntry) = self._get_utf16(lang, cntry);

        (str_from_char_2(lang), str_from_char_2(cntry))
    }

    pub fn get_translation_count(&self) -> usize {
        self.entries.len()
    }
}

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
