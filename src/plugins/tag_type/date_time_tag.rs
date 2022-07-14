use std::{any::Any, io::Result};

use crate::{io::IOHandler, state::Context, types::DateTimeNumber};

pub fn read(
    _context: &mut Context,
    io: &mut dyn IOHandler,
    _size_of_tag: usize,
) -> Result<(usize, Box<dyn Any>)> {
    let date_time = DateTimeNumber {
        year: io.read_u16()?,
        month: io.read_u16()?,
        day: io.read_u16()?,
        hours: io.read_u16()?,
        minutes: io.read_u16()?,
        seconds: io.read_u16()?,
    };
    let new_date_time: chrono::NaiveDateTime = date_time.into();
    Ok((1, Box::new(new_date_time)))
}
/// ptr MUST be &Box\<chrono::NaiveDateTime\>
pub fn write(
    _context: &mut Context,
    io: &mut dyn IOHandler,
    ptr: &Box<dyn Any>,
    _num_items: usize,
) -> Result<()> {
    let date_time = ptr.downcast_ref::<chrono::NaiveDateTime>().unwrap();
    let new_date_time: DateTimeNumber = (*date_time).into();

    io.write_u16(new_date_time.year)?;
    io.write_u16(new_date_time.month)?;
    io.write_u16(new_date_time.day)?;
    io.write_u16(new_date_time.hours)?;
    io.write_u16(new_date_time.minutes)?;
    io.write_u16(new_date_time.seconds)
}
