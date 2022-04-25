use chrono::{Datelike, Timelike};

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

impl From<DateTimeNumber> for chrono::NaiveDateTime {
    fn from(d: DateTimeNumber) -> Self {
        chrono::NaiveDate::from_ymd(d.year as i32, d.month as u32, d.day as u32).and_hms(
            d.hours as u32,
            d.minutes as u32,
            d.seconds as u32,
        )
    }
}
impl From<chrono::NaiveDateTime> for DateTimeNumber {
    fn from(d: chrono::NaiveDateTime) -> Self {
        let time = d.time();

        let (_, year) = d.year_ce();
        Self {
            year: year as u16,
            month: d.month() as u16,
            day: d.day() as u16,
            hours: time.hour() as u16,
            minutes: time.minute() as u16,
            seconds: time.second() as u16,
        }
    }
}
