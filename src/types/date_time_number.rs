/// ICC date time
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct DateTimeNumber {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub hours: u16,
    pub minutes: u16,
    pub seconds: u16,
}
