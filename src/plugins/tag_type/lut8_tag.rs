use std::{
    any::Any,
    io::{ErrorKind, Result},
};

use crate::{
    io::IOHandler,
    math::{from_16_to_8, from_8_to_16},
    state::{Context, ErrorCode},
    types::{
        signatures::stage, Matrix, Pipeline, Stage, StageClutData, StageLoc, StageMatrixData,
        StageToneCurveData, ToneCurve, Vec3, MAX_CHANNELS,
    },
};

pub fn read(
    context: &mut Context,
    io: &mut dyn IOHandler,
    _size_of_tag: usize,
) -> Result<(usize, Box<dyn Any>)> {
    let input_channels = io.read_u8()?;
    let output_channels = io.read_u8()?;
    let clut_points = io.read_u8()?; // 255 maximum

    // Impossible value, 0 for no CLUT and then 2 at least
    if clut_points == 1 {
        return Err(ErrorKind::InvalidData.into());
    }

    // Padding
    _ = io.read_u8()?;

    // Do some checking
    if input_channels == 0 || input_channels as usize > MAX_CHANNELS {
        return Err(ErrorKind::InvalidData.into());
    }
    if output_channels == 0 || output_channels as usize > MAX_CHANNELS {
        return Err(ErrorKind::InvalidData.into());
    }
    let mut new_lut = Pipeline::new(input_channels as u32, output_channels as u32);

    let matrix = Matrix(
        Vec3(io.read_s15f16()?, io.read_s15f16()?, io.read_s15f16()?),
        Vec3(io.read_s15f16()?, io.read_s15f16()?, io.read_s15f16()?),
        Vec3(io.read_s15f16()?, io.read_s15f16()?, io.read_s15f16()?),
    );

    // Only operates on 3 channels
    if input_channels == 3 && !matrix.is_identity() {
        let mat: [f64; 9] = matrix.into();
        if let Some(mpe) = Stage::alloc_matrix(3, 3, &mat, &[]) {
            new_lut.insert_stage(StageLoc::AtEnd, mpe);
        } else {
            return Err(ErrorKind::InvalidData.into());
        }
    }

    // Get input tables
    read_8bit_tables(context, io, &mut new_lut, input_channels as u32)?;

    // Get 3D CLUT
    let num_tab_size = ToneCurve::uipow(
        output_channels as u32,
        clut_points as u32,
        input_channels as u32,
    );
    if num_tab_size == u32::MAX {
        return Err(ErrorKind::InvalidData.into());
    }
    if num_tab_size > 0 {
        let mut ptr_w = vec![0u16; num_tab_size as usize];

        let mut temp = vec![0u8; num_tab_size as usize];

        io.read(temp.as_mut_slice())?;

        for i in 0..num_tab_size as usize {
            ptr_w[i] = from_8_to_16(temp[i]);
        }

        if let Some(mpe) = Stage::alloc_clut_16bit(
            clut_points as u32,
            input_channels as u32,
            output_channels as u32,
            ptr_w.as_mut_slice(),
        ) {
            new_lut.insert_stage(StageLoc::AtEnd, mpe);
        } else {
            return Err(ErrorKind::InvalidData.into());
        }
    }

    // Get output tables
    read_8bit_tables(context, io, &mut new_lut, output_channels as u32)?;

    Ok((1, Box::new(new_lut)))
}

/// ptr MUST be &Box<Pipeline>
pub fn write(
    context: &mut Context,
    io: &mut dyn IOHandler,
    ptr: &Box<dyn Any>,
    _num_items: usize,
) -> Result<()> {
    let mut mat_mpe: Option<&StageMatrixData> = None;
    let mut clut: Option<&StageClutData> = None;
    let mut pre_mpe: Option<&StageToneCurveData> = None;
    let mut post_mpe: Option<&StageToneCurveData> = None;

    let new_lut = if let Some(result) = ptr.downcast_ref::<Pipeline>() {
        result
    } else {
        return Err(ErrorKind::InvalidInput.into());
    };

    // Disassemble the LUT into components.
    if !new_lut.elements.is_empty() {
        let mut iter = new_lut.elements.iter();
        let mut stage = iter.next();
        if let Some(mpe) = stage {
            if mpe.r#type == stage::MATRIX_ELEM_TYPE {
                if mpe.input_channels != 3 || mpe.output_channels != 3 {
                    return Err(ErrorKind::InvalidInput.into());
                }
                mat_mpe = mpe.data.as_ref().unwrap().downcast_ref::<StageMatrixData>();
                stage = iter.next();
            }
        }
        if let Some(mpe) = stage {
            if mpe.r#type == stage::CURVE_SET_ELEM_TYPE {
                pre_mpe = mpe
                    .data
                    .as_ref()
                    .unwrap()
                    .downcast_ref::<StageToneCurveData>();
                stage = iter.next();
            }
        }
        if let Some(mpe) = stage {
            if mpe.r#type == stage::C_LUT_ELEM_TYPE {
                clut = mpe.data.as_ref().unwrap().downcast_ref::<StageClutData>();
                stage = iter.next();
            }
        }
        if let Some(mpe) = stage {
            if mpe.r#type == stage::CURVE_SET_ELEM_TYPE {
                post_mpe = mpe
                    .data
                    .as_ref()
                    .unwrap()
                    .downcast_ref::<StageToneCurveData>();
                stage = iter.next();
            }
        }

        // That should be all
        if stage.is_some() {
            let error = "LUT is not suitable to be saved as LUT16";
            context.signal_error(ErrorCode::UnknownExtension, error);
            return Err(ErrorKind::InvalidInput.into());
        }
    }

    let clut_points = if clut.is_none() {
        0
    } else {
        clut.unwrap().params[0].num_samples[0] as u8
    };

    let input_channels = new_lut.input_channels as u8;
    let output_channels = new_lut.output_channels as u8;

    io.write_u8(input_channels)?;
    io.write_u8(output_channels)?;
    io.write_u8(clut_points)?;
    io.write_u8(0)?; // Padding

    if let Some(mat_mpe) = mat_mpe {
        for value in mat_mpe.double.iter() {
            io.write_s15f16(*value)?;
        }
    } else {
        io.write_s15f16(1.0)?;
        io.write_s15f16(0.0)?;
        io.write_s15f16(0.0)?;
        io.write_s15f16(0.0)?;
        io.write_s15f16(1.0)?;
        io.write_s15f16(0.0)?;
        io.write_s15f16(0.0)?;
        io.write_s15f16(0.0)?;
        io.write_s15f16(1.0)?;
    }

    // The prelinearization table
    write_8bit_tables(context, io, pre_mpe)?;

    let num_tab_size = ToneCurve::uipow(
        output_channels as u32,
        clut_points as u32,
        input_channels as u32,
    );
    if num_tab_size == u32::MAX {
        return Err(ErrorKind::InvalidInput.into());
    }
    if num_tab_size > 0 {
        // The 3D LUT
        if let Some(clut) = clut {
            for j in 0..num_tab_size as usize {
                io.write_u8(from_16_to_8(clut.tab.as_u16().unwrap()[j]))?;
            }
        }
    }

    // The postlinearization table
    write_8bit_tables(context, io, post_mpe)?;

    Ok(())
}

fn read_8bit_tables(
    context: &mut Context,
    io: &mut dyn IOHandler,
    lut: &mut Pipeline,
    num_channels: u32,
) -> Result<()> {
    if num_channels as usize > MAX_CHANNELS || num_channels <= 0 {
        return Err(ErrorKind::InvalidData.into());
    }

    let mut tables: [ToneCurve; MAX_CHANNELS] = Default::default();

    for i in 0..num_channels as usize {
        if let Some(mut table) = ToneCurve::build_tabulated_16(context, 256, &[]) {
            io.read_u16_array(table.table16.as_mut())?;
            tables[i] = table;
        } else {
            return Err(ErrorKind::InvalidData.into());
        }
    }

    let mut temp = [0u8; 256];

    for i in 0..num_channels as usize {
        io.read(&mut temp)?;

        for j in 0..256 {
            tables[i].table16[j] = from_8_to_16(temp[j]);
        }
    }

    if let Some(mpe) = Stage::alloc_tone_curves(num_channels, &tables) {
        lut.insert_stage(StageLoc::AtEnd, mpe)
    } else {
        return Err(ErrorKind::InvalidData.into());
    }

    Ok(())
}

fn write_8bit_tables(
    context: &mut Context,
    io: &mut dyn IOHandler,
    tables: Option<&StageToneCurveData>,
) -> Result<()> {
    if let Some(tables) = tables {
        for curve in tables.curves.iter() {
            // Usual case of identity curves
            if (curve.table16.len() == 2) && (curve.table16[0] == 0) && (curve.table16[1] == 65535)
            {
                for j in 0..256 {
                    io.write_u8(j as u8)?;
                }
            } else if curve.table16.len() != 256 {
                let error = "LUT8 needs 256 entries on prelinearization";
                context.signal_error(ErrorCode::UnknownExtension, error);
                return Err(ErrorKind::InvalidInput.into());
            } else {
                for j in 0..256 {
                    io.write_u8(from_16_to_8(curve.table16[j]))?;
                }
            }
        }
    }

    Ok(())
}
