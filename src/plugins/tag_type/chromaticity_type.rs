use std::{
    any::Any,
    io::{ErrorKind, Result},
};

use crate::{
    io::IOHandler,
    state::Context,
    types::{CIExyY, CIExyYTriple},
};

pub fn read(
    _context: &mut Context,
    io: &mut dyn IOHandler,
    size_of_tag: usize,
) -> Result<(usize, Box<dyn Any>)> {
    let mut chans = io.read_u16()?;

    // Let's recover from a bug introduced in early versions of the original lcms1
    if chans == 0 && size_of_tag == 32 {
        _ = io.read_u16();
        chans = io.read_u16()?;
    }

    if chans != 3 {
        return Err(ErrorKind::InvalidData.into());
    }
    _ = io.read_u16()?;
    let red = CIExyY {
        x: io.read_s15f16()?,
        y: io.read_s15f16()?,
        Y: 1.0,
    };
    let green = CIExyY {
        x: io.read_s15f16()?,
        y: io.read_s15f16()?,
        Y: 1.0,
    };
    let blue = CIExyY {
        x: io.read_s15f16()?,
        y: io.read_s15f16()?,
        Y: 1.0,
    };

    Ok((1, Box::new(CIExyYTriple { red, green, blue })))
}
/// ptr MUST be a &Box<CIExyYTriple>
pub fn write(
    _context: &mut Context,
    io: &mut dyn IOHandler,
    ptr: &Box<dyn Any>,
    _num_items: usize,
) -> Result<()> {
    let value = ptr.downcast_ref::<CIExyYTriple>().unwrap();

    io.write_u16(3)?;
    io.write_u16(0)?;

    save_one_chromaticity(io, value.red.x, value.red.y)?;
    save_one_chromaticity(io, value.green.x, value.green.y)?;
    save_one_chromaticity(io, value.blue.x, value.blue.y)
}
fn save_one_chromaticity(io: &mut dyn IOHandler, x: f64, y: f64) -> Result<()> {
    io.write_s15f16(x)?;
    io.write_s15f16(y)
}
