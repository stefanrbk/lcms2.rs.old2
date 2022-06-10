use std::sync::{Arc, Mutex};

use crate::state::Context;

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
    context: Arc<Mutex<Context>>,
    colorant_count: u32,
}
