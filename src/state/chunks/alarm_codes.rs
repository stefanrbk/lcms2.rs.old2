use crate::types::MAX_CHANNELS;

#[derive(Copy, Clone, Debug)]
pub struct AlarmCodesChunk {
    pub alarm_codes: [u16; MAX_CHANNELS],
}

impl AlarmCodesChunk {
    pub fn new(alarm_codes: [u16; MAX_CHANNELS]) -> Self {
        Self { alarm_codes }
    }
}

impl Default for AlarmCodesChunk {
    fn default() -> Self {
        Self {
            alarm_codes: [
                0x7F00, 0x7F00, 0x7F00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        }
    }
}
