use std::{
    any::Any,
    io::{ErrorKind, Result, SeekFrom},
};

use crate::{
    io::IOHandler,
    math::from_8_to_16,
    state::{Context, ErrorCode},
    types::{
        signatures::tag_type, Pipeline, Stage, StageClutData, StageLoc, ToneCurve, MAX_CHANNELS,
    },
};

use super::{curve_type, read_set_of_curves, read_type_base, read_clut, read_matrix};

pub fn read(
    context: &mut Context,
    io: &mut dyn IOHandler,
    _size_of_tag: usize,
) -> Result<(usize, Box<dyn Any>)> {
    let base_offset = io.tell()? - 8; // sizeof(_cmsTagBase)

    let input_chan = io.read_u8()? as usize;
    let output_chan = io.read_u8()? as usize;

    _ = io.read_u16()?;

    let offset_b = io.read_u32()? as usize;
    let offset_mat = io.read_u32()? as usize;
    let offset_m = io.read_u32()? as usize;
    let offset_c = io.read_u32()? as usize;
    let offset_a = io.read_u32()? as usize;

    if input_chan == 0 || input_chan > MAX_CHANNELS {
        return Err(ErrorKind::InvalidData.into());
    }

    if output_chan == 0 || output_chan > MAX_CHANNELS {
        return Err(ErrorKind::InvalidData.into());
    }

    let mut new_lut = Pipeline::new(input_chan as u32, output_chan as u32);

    if offset_a != 0 {
        new_lut.insert_stage(
            StageLoc::AtEnd,
            read_set_of_curves(context, io, base_offset + offset_a, input_chan)?,
        )
    }
    if offset_c != 0 {
        new_lut.insert_stage(
            StageLoc::AtEnd,
            read_clut(context, io, base_offset + offset_c, input_chan, output_chan)?,
        )
    }
    if offset_m != 0 {
        new_lut.insert_stage(
            StageLoc::AtEnd,
            read_set_of_curves(context, io, base_offset + offset_m, output_chan)?,
        )
    }
    if offset_mat != 0 {
        new_lut.insert_stage(
            StageLoc::AtEnd,
            read_matrix(io, base_offset + offset_mat)?,
        )
    }
    if offset_b != 0 {
        new_lut.insert_stage(
            StageLoc::AtEnd,
            read_set_of_curves(context, io, base_offset + offset_b, output_chan)?,
        )
    }

    Ok((1, Box::new(new_lut)))
}
