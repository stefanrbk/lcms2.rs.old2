use std::{
    any::Any,
    io::{ErrorKind, Result},
    mem::size_of,
};

use crate::{io::IOHandler, state::Context, types::ICCData};

pub fn read(
    _context: &mut Context,
    io: &mut dyn IOHandler,
    size_of_tag: usize,
) -> Result<(usize, Box<dyn Any>)> {
    if size_of_tag < size_of::<u32>() {
        return Err(ErrorKind::InvalidInput.into());
    }

    let len_of_data = size_of_tag - size_of::<u32>();
    let flags = io.read_u32()?;
    let mut buffer = vec![0u8; len_of_data];
    io.read(buffer.as_mut_slice())?;

    Ok((
        1,
        Box::new(ICCData {
            flag: flags,
            data: buffer.into_boxed_slice(),
        }),
    ))
}
/// ptr MUST be &Box<ICCData>
pub fn write(
    _context: &mut Context,
    io: &mut dyn IOHandler,
    ptr: &Box<dyn Any>,
    _num_items: usize,
) -> Result<()> {
    let data = ptr.downcast_ref::<ICCData>().unwrap();

    io.write_u32(data.flag)?;
    io.write(&data.data)
}
