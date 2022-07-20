use std::{
    any::Any,
    io::{ErrorKind, Result, SeekFrom},
};

use crate::{
    io::IOHandler,
    state::{Context, ErrorCode},
    types::{
        signatures::{stage, tag_type},
        Pipeline, Stage, StageLoc, MAX_CHANNELS,
    },
};

use super::{read_clut, read_matrix, read_set_of_curves, write_clut, write_set_of_curves, write_matrix};

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
        new_lut.insert_stage(StageLoc::AtEnd, read_matrix(io, base_offset + offset_mat)?)
    }
    if offset_b != 0 {
        new_lut.insert_stage(
            StageLoc::AtEnd,
            read_set_of_curves(context, io, base_offset + offset_b, output_chan)?,
        )
    }

    Ok((1, Box::new(new_lut)))
}
/// ptr MUST be a &Box\<Pipeline\>
pub fn write(
    context: &mut Context,
    io: &mut dyn IOHandler,
    ptr: &Box<dyn Any>,
    _num_items: usize,
) -> Result<()> {
    let lut = ptr.downcast_ref::<Pipeline>().unwrap();

    let mut a: Option<Stage> = None;
    let mut b: Option<Stage> = None;
    let mut clut: Option<Stage> = None;
    let mut m: Option<Stage> = None;
    let mut matrix: Option<Stage> = None;

    let base_offset = io.tell()?;

    if lut.elements.len() != 0 {
        if lut
            .check_and_retrieve_stages(1, &[(stage::CURVE_SET_ELEM_TYPE, &mut b)])
            .is_none()
        {
            if lut
                .check_and_retrieve_stages(
                    3,
                    &[
                        (stage::CURVE_SET_ELEM_TYPE, &mut m),
                        (stage::MATRIX_ELEM_TYPE, &mut matrix),
                        (stage::CURVE_SET_ELEM_TYPE, &mut b),
                    ],
                )
                .is_none()
            {
                if lut
                    .check_and_retrieve_stages(
                        3,
                        &[
                            (stage::CURVE_SET_ELEM_TYPE, &mut a),
                            (stage::C_LUT_ELEM_TYPE, &mut clut),
                            (stage::CURVE_SET_ELEM_TYPE, &mut b),
                        ],
                    )
                    .is_none()
                {
                    if lut
                        .check_and_retrieve_stages(
                            5,
                            &[
                                (stage::CURVE_SET_ELEM_TYPE, &mut a),
                                (stage::C_LUT_ELEM_TYPE, &mut clut),
                                (stage::CURVE_SET_ELEM_TYPE, &mut m),
                                (stage::MATRIX_ELEM_TYPE, &mut matrix),
                                (stage::CURVE_SET_ELEM_TYPE, &mut b),
                            ],
                        )
                        .is_none()
                    {
                        context.signal_error(
                            ErrorCode::UnknownExtension,
                            "LUT is not suitable to be saved as LutAToB",
                        );
                        return Err(ErrorKind::InvalidInput.into());
                    }
                }
            }
        }
    }

    // Get input, output channels
    let input_chan = lut.input_channels as u8;
    let output_chan = lut.output_channels as u8;

    // Write channel count
    io.write_u8(input_chan)?;
    io.write_u8(output_chan)?;
    io.write_u16(0)?;

    // Keep directory to be filled later
    let directory_pos = io.tell()?;

    // Write the directory
    for _ in 0..5 {
        io.write_u32(0)?;
    }

    let offset_a = if let Some(a) = a {
        let result = io.tell()? - base_offset;
        write_set_of_curves(context, io, tag_type::PARAMETRIC_CURVE, &a)?;
        result as u32
    } else {
        0
    };
    let offset_c = if let Some(clut) = clut {
        let result = io.tell()? - base_offset;
        write_clut(context, io, if lut.save_as_8_bits { 1 } else { 2 }, &clut)?;
        result as u32
    } else {
        0
    };
    let offset_m = if let Some(m) = m {
        let result = io.tell()? - base_offset;
        write_set_of_curves(context, io, tag_type::PARAMETRIC_CURVE, &m)?;
        result as u32
    } else {
        0
    };
    let offset_mat = if let Some(matrix) = matrix {
        let result = io.tell()? - base_offset;
        write_matrix(io, &matrix)?;
        result as u32
    } else {
        0
    };
    let offset_b = if let Some(b) = b {
        let result = io.tell()? - base_offset;
        write_set_of_curves(context, io, tag_type::PARAMETRIC_CURVE, &b)?;
        result as u32
    } else {
        0
    };
    let current_pos = io.tell()?;

    io.seek(SeekFrom::Start(directory_pos as u64))?;

    io.write_u32(offset_b)?;
    io.write_u32(offset_mat)?;
    io.write_u32(offset_m)?;
    io.write_u32(offset_c)?;
    io.write_u32(offset_a)?;

    io.seek(SeekFrom::Start(current_pos as u64))
}
