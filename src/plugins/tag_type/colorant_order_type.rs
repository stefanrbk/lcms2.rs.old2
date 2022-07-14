use std::{any::Any, io::Result};

use crate::{io::IOHandler, state::Context};

pub fn read(
    _context: &mut Context,
    io: &mut dyn IOHandler,
    _size_of_tag: usize,
) -> Result<(usize, Box<dyn Any>)> {
    let count = io.read_u32()? as usize;

    let mut order = Vec::new();
    for _ in [0..count] {
        order.push(io.read_u8()?);
    }

    Ok((1, Box::new(order)))
}
/// ptr MUST be &Box<Vec<u8>>
pub fn write(
    _context: &mut Context,
    io: &mut dyn IOHandler,
    ptr: &Box<dyn Any>,
    _num_items: usize,
) -> Result<()> {
    let ptr = ptr.downcast_ref::<Vec<u8>>().unwrap();
    let len = ptr.len() as u32;

    io.write_u32(len)?;

    for value in ptr.iter() {
        io.write_u8(*value)?;
    }
    Ok(())
}