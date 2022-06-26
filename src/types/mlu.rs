pub struct MluEntry {
    language: [char; 2],
    country: [char; 2],

    value: String,
}

pub struct Mlu {
    entries: Vec<MluEntry>,
}
// &mut Context must be passed in for all functions involving Mlu
