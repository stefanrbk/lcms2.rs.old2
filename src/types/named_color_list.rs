use std::ops::Deref;

use super::MAX_CHANNELS;

pub struct NamedColor {
    name: String,
    pcs: [u16; 3],
    device_colorant: [u16; MAX_CHANNELS],
}

impl NamedColor {
    pub fn new(name: String, pcs: [u16; 3], device_colorant: [u16; MAX_CHANNELS]) -> Self { Self { name, pcs, device_colorant } }
}

pub struct NamedColorList {
    prefix: String,
    suffix: String,
    list: Vec<NamedColor>,
}
// &mut Context must be passed in for all functions involving NamedColorList
impl NamedColorList {
    pub fn new(prefix: impl Into<String>, suffix: impl Into<String>) -> Self {
        Self { prefix: prefix.into(), suffix: suffix.into(), list: Vec::new() }
    }
    pub fn append(&mut self, color: NamedColor) {
        self.list.push(color);
    }
    pub fn len(&self) -> usize {
        self.list.len()
    }
    pub fn get(&self, index: usize) -> Option<(&String, &NamedColor, &String)> {
        if index >= self.list.len() {
            return None;
        }
        Some((&self.prefix, &self.list[index], &self.suffix))
    }
    pub fn get_mut(&mut self, index: usize) -> Option<(&mut String, &mut NamedColor, &mut String)> {
        if index >= self.list.len() {
            return None;
        }
        Some((&mut self.prefix, &mut self.list[index], &mut self.suffix))
    }
    pub fn find(&self, name: impl Into<String>) -> Option<usize> {
        let name: String = name.into();
        let mut i = 0usize;
        for item in self.list.iter() {
            if item.name == name {
                return Some(i);
            }
            i += 1;
        }
        None
    }
}
impl Deref for NamedColorList {
    type Target = Vec<NamedColor>;

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}
