use std::sync::{Arc, Mutex};

use crate::state::Context;

pub struct MluEntry {
    language: [char; 2],
    country: [char; 2],

    value: String,
}

pub struct Mlu {
    context: Arc<Mutex<Context>>,
    entries: Vec<MluEntry>,
}
