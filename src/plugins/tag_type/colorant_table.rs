use std::{
    any::Any,
    io::{ErrorKind, Result},
};

use crate::{
    io::IOHandler,
    state::{Context, ErrorCode},
    types::{NamedColor, NamedColorList, MAX_CHANNELS},
};

pub fn colorant_table_read(
    context: &mut Context,
    io: &mut dyn IOHandler,
    _size_of_tag: usize,
) -> Result<(usize, Box<dyn Any>)> {
    let count = io.read_u32()? as usize;

    if count > MAX_CHANNELS {
        context.signal_error(ErrorCode::Range, format!("Too many colorants '{}'", count));
        return Err(ErrorKind::InvalidData.into());
    }

    let mut list = NamedColorList::new("", "");

    for _ in 0..count {
        let mut raw_name = [0u8; 32];
        let mut pcs = [0u16; 3];

        io.read(&mut raw_name)?;
        io.read_u16_array(&mut pcs)?;

        let mut idx = 32usize;
        for i in 0..32 {
            if raw_name[i] == 0 {
                idx = i;
                break;
            }
        }

        match String::from_utf8(raw_name[0..idx].to_vec()) {
            Ok(name) => list.append(NamedColor::new(name, pcs, Default::default())),
            Err(_) => return Err(ErrorKind::InvalidData.into()),
        }
    }

    Ok((1, Box::new(list)))
}
/// ptr MUST be &Box<NamedColorList>
pub fn colorant_table_write(
    _context: &mut Context,
    io: &mut dyn IOHandler,
    ptr: &Box<dyn Any>,
    _num_items: usize,
) -> Result<()> {
    let ptr = ptr.downcast_ref::<NamedColorList>().unwrap();
    let len = ptr.len() as u32;

    io.write_u32(len)?;

    for value in ptr.iter() {
        let name = value.name.as_bytes();
        let mut bytes = [0u8; 32];
        let len = name.len();

        if len >= 32 {
            bytes.copy_from_slice(&name[0..32]);
        } else {
            bytes[0..len].copy_from_slice(&name);
        }

        io.write(&bytes)?;
        io.write_u16_array(&value.pcs)?;
    }
    Ok(())
}
