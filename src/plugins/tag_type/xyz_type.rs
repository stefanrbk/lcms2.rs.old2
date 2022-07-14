use std::{any::Any, io::Result};

use crate::{
    io::IOHandler,
    state::Context,
    types::{signatures::tag_type, Signature, CIEXYZ},
};

pub fn read(
    _context: &mut Context,
    io: &mut dyn IOHandler,
    _size_of_tag: usize,
) -> Result<(usize, Box<dyn Any>)> {
    Ok((1, Box::new(io.read_xyz()?)))
}
/// ptr MUST be &Box<CIEXYZ>
pub fn write(
    _context: &mut Context,
    io: &mut dyn IOHandler,
    ptr: &Box<dyn Any>,
    _num_items: usize,
) -> Result<()> {
    io.write_xyz(*ptr.downcast_ref::<CIEXYZ>().unwrap())
}

pub fn decide(_version: f64, _data: &Box<dyn Any>) -> Signature {
    tag_type::XYZ
}
