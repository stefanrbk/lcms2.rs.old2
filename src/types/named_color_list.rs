use super::MAX_CHANNELS;

pub struct NamedColor {
    name: String,
    pcs: [u16; 3],
    device_colorant: [u16; MAX_CHANNELS],
}

pub struct NamedColorList {
    prefix: String,
    suffix: String,
    list: Vec<NamedColor>,
    colorant_count: u32,
}
// &mut Context must be passed in for all functions involving NamedColorList
