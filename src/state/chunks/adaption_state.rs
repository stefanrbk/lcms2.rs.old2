#[derive(Copy, Clone, Debug)]
pub struct AdaptionStateChunk {
    pub adaption_state: f64,
}

impl AdaptionStateChunk {
    pub fn new(adaption_state: f64) -> Self {
        Self { adaption_state }
    }
}

impl Default for AdaptionStateChunk {
    fn default() -> Self {
        Self {
            adaption_state: 1.0,
        }
    }
}
