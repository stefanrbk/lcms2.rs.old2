#![allow(non_snake_case)]
use std::{
    any::Any,
    fmt::Debug,
    io::{ErrorKind, Result, SeekFrom},
    mem::size_of,
};

use crate::{
    io::IOHandler,
    math::{f64_to_u8f8, from_16_to_8, from_8_to_16, u8f8_to_f64},
    state::{Context, ErrorCode},
    types::{
        signatures::{stage, tag_type},
        DateTimeNumber, ICCData, Matrix, Pipeline, Signature, Stage, StageClutData, StageLoc,
        StageMatrixData, StageToneCurveData, ToneCurve, Vec3, CIEXYZ, MAX_CHANNELS,
    },
};

pub type TagTypeList = Vec<TypeHandler>;

pub type TagTypeReader = fn(
    context: &mut Context,
    io: &mut dyn IOHandler,
    size_of_tag: usize,
) -> Result<(usize, Box<dyn Any>)>;

pub type TagTypeWriter = fn(
    context: &mut Context,
    io: &mut dyn IOHandler,
    ptr: &Box<dyn Any>,
    num_items: usize,
) -> Result<()>;

pub type TypeDecider = fn(version: f64, data: &Box<dyn Any>) -> Signature;

#[derive(Clone)]
pub struct TypeHandler {
    pub(crate) signature: Signature,
    pub(crate) icc_version: u32,
    pub(crate) read: TagTypeReader,
    pub(crate) write: TagTypeWriter,
}

impl Debug for TypeHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeHandler")
            .field("signature", &self.signature)
            .field("icc_version", &self.icc_version)
            .field("read", &"[Function Ptr]")
            .field("write", &"[Function Ptr]")
            .finish()
    }
}

impl TypeHandler {
    pub fn data_read(
        &self,
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
    pub fn date_time_read(
        &self,
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
    pub fn lut16_read(
        &self,
        context: &mut Context,
        io: &mut dyn IOHandler,
        _size_of_tag: usize,
    ) -> Result<(usize, Box<dyn Any>)> {
        let input_channels = io.read_u8()?;
        let output_channels = io.read_u8()?;
        let clut_points = io.read_u8()?; // 255 maximum

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

        let input_entries = io.read_u16()?;
        let output_entries = io.read_u16()?;

        if input_entries > 0x7FFF || output_entries > 0x7FFF {
            return Err(ErrorKind::InvalidData.into());
        }
        // Impossible value, 0 for no CLUT and then 2 at least
        if clut_points == 1 {
            return Err(ErrorKind::InvalidData.into());
        }

        // Get input tables
        read_16bit_tables(
            context,
            io,
            &mut new_lut,
            input_channels as u32,
            input_entries as u32,
        )?;

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
            let mut t = vec![0u16; num_tab_size as usize];

            io.read_u16_array(t.as_mut_slice())?;

            if let Some(mpe) = Stage::alloc_clut_16bit(
                clut_points as u32,
                input_channels as u32,
                output_channels as u32,
                t.as_mut_slice(),
            ) {
                new_lut.insert_stage(StageLoc::AtEnd, mpe);
            } else {
                return Err(ErrorKind::InvalidData.into());
            }
        }

        // Get output tables
        read_16bit_tables(
            context,
            io,
            &mut new_lut,
            output_channels as u32,
            output_entries as u32,
        )?;

        Ok((1, Box::new(new_lut)))
    }
    pub fn lut8_read(
        &self,
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

    pub fn XYZ_read(
        &self,
        _context: &mut Context,
        io: &mut dyn IOHandler,
        _size_of_tag: usize,
    ) -> Result<(usize, Box<dyn Any>)> {
        Ok((1, Box::new(io.read_xyz()?)))
    }

    /// ptr MUST be &Box<ICCData>
    pub fn data_write(
        &self,
        _context: &mut Context,
        io: &mut dyn IOHandler,
        ptr: Box<dyn Any>,
        _num_items: usize,
    ) -> Result<()> {
        let data = ptr.downcast::<ICCData>().unwrap();

        io.write_u32(data.flag)?;
        io.write(&data.data)
    }
    /// ptr MUST be &Box\<chrono::NaiveDateTime\>
    pub fn date_time_write(
        &self,
        _context: &mut Context,
        io: &mut dyn IOHandler,
        ptr: Box<dyn Any>,
        _num_items: usize,
    ) -> Result<()> {
        let date_time = ptr.downcast::<chrono::NaiveDateTime>().unwrap();
        let new_date_time: DateTimeNumber = (*date_time.as_ref()).into();

        io.write_u16(new_date_time.year)?;
        io.write_u16(new_date_time.month)?;
        io.write_u16(new_date_time.day)?;
        io.write_u16(new_date_time.hours)?;
        io.write_u16(new_date_time.minutes)?;
        io.write_u16(new_date_time.seconds)
    }
    /// ptr MUST be &Box<ICCData>
    pub fn lut16_write(
        &self,
        context: &mut Context,
        io: &mut dyn IOHandler,
        ptr: Box<dyn Any>,
        _num_items: usize,
    ) -> Result<()> {
        let mut mat_mpe: Option<&StageMatrixData> = None;
        let mut clut: Option<&StageClutData> = None;
        let mut pre_mpe: Option<&StageToneCurveData> = None;
        let mut post_mpe: Option<&StageToneCurveData> = None;

        let new_lut = if let Ok(result) = ptr.downcast::<Pipeline>() {
            result
        } else {
            return Err(ErrorKind::InvalidInput.into());
        };

        // Disassemble the LUT into components.
        if !new_lut.elements.is_empty() {
            let mut iter = new_lut.elements.iter();
            let mut stage = iter.next();
            if stage.is_some() {
                let mpe = stage.unwrap();
                if mpe.r#type == stage::MATRIX_ELEM_TYPE {
                    mat_mpe = mpe.data.as_ref().unwrap().downcast_ref::<StageMatrixData>();
                    if mpe.input_channels != 3 || mpe.output_channels != 3 {
                        return Err(ErrorKind::InvalidInput.into());
                    }
                    stage = iter.next();
                }
            }
            if stage.is_some() {
                let mpe = stage.unwrap();
                if mpe.r#type == stage::CURVE_SET_ELEM_TYPE {
                    pre_mpe = mpe
                        .data
                        .as_ref()
                        .unwrap()
                        .downcast_ref::<StageToneCurveData>();
                    stage = iter.next();
                }
            }
            if stage.is_some() {
                let mpe = stage.unwrap();
                if mpe.r#type == stage::C_LUT_ELEM_TYPE {
                    clut = mpe.data.as_ref().unwrap().downcast_ref::<StageClutData>();
                    stage = iter.next();
                }
            }
            if stage.is_some() {
                let mpe = stage.unwrap();
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

        let input_channels = new_lut.input_channels as u8;
        let output_channels = new_lut.output_channels as u8;

        let clut_points = if clut.is_none() {
            0
        } else {
            clut.unwrap().params[0].num_samples[0] as u8
        };

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

        io.write_u16(if let Some(pre_mpe) = pre_mpe {
            pre_mpe.curves[0].table16.len() as u16
        } else {
            2
        })?;

        io.write_u16(if let Some(post_mpe) = post_mpe {
            post_mpe.curves[0].table16.len() as u16
        } else {
            2
        })?;

        // The prelinearization table
        if let Some(pre_mpe) = pre_mpe {
            write_16bit_tables(io, pre_mpe)?;
        } else {
            for _ in 0..2 {
                io.write_u16(0)?;
                io.write_u16(0xFFFF)?;
            }
        }

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
                io.write_u16_array(clut.tab.as_u16().unwrap())?;
            }
        }

        // The prelinearization table
        if let Some(post_mpe) = post_mpe {
            write_16bit_tables(io, post_mpe)?;
        } else {
            for _ in 0..2 {
                io.write_u16(0)?;
                io.write_u16(0xFFFF)?;
            }
        }

        Ok(())
    }
    /// ptr MUST be &Box<Pipeline>
    pub fn lut8_write(
        &self,
        context: &mut Context,
        io: &mut dyn IOHandler,
        ptr: Box<dyn Any>,
        _num_items: usize,
    ) -> Result<()> {
        let mut mat_mpe: Option<&StageMatrixData> = None;
        let mut clut: Option<&StageClutData> = None;
        let mut pre_mpe: Option<&StageToneCurveData> = None;
        let mut post_mpe: Option<&StageToneCurveData> = None;

        let new_lut = if let Ok(result) = ptr.downcast::<Pipeline>() {
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

    /// ptr MUST be &Box<CIEXYZ>
    pub fn XYZ_write(
        &self,
        _context: &mut Context,
        io: &mut dyn IOHandler,
        ptr: Box<dyn Any>,
        _num_items: usize,
    ) -> Result<()> {
        io.write_xyz(*ptr.downcast::<CIEXYZ>().unwrap())
    }

    pub fn decide_XYZ(_version: f64, _data: &Box<dyn Any>) -> Signature {
        tag_type::XYZ
    }
}

pub fn read_16bit_tables(
    context: &mut Context,
    io: &mut dyn IOHandler,
    lut: &mut Pipeline,
    num_channels: u32,
    num_entries: u32,
) -> Result<()> {
    // Maybe an empty table? (this is an extension)
    if num_entries <= 0 {
        return Ok(());
    }

    // Check for malicious profiles
    if num_entries < 2 {
        return Err(ErrorKind::InvalidData.into());
    }
    if num_channels as usize > MAX_CHANNELS {
        return Err(ErrorKind::InvalidData.into());
    }

    let mut tables: [ToneCurve; MAX_CHANNELS] = Default::default();

    for i in 0..num_channels as usize {
        if let Some(mut table) = ToneCurve::build_tabulated_16(context, num_entries as usize, &[]) {
            io.read_u16_array(table.table16.as_mut())?;
            tables[i] = table;
        } else {
            return Err(ErrorKind::InvalidData.into());
        }
    }

    // Add the table (which may certainly be an identity, but this is up to the optimizer, not the reading code)
    if let Some(mpe) = Stage::alloc_tone_curves(num_channels, &tables) {
        lut.insert_stage(StageLoc::AtEnd, mpe)
    } else {
        return Err(ErrorKind::InvalidData.into());
    }

    Ok(())
}

pub fn read_8bit_tables(
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

fn read_matrix(io: &mut dyn IOHandler, offset: usize) -> Result<Stage> {
    let mut d_mat = [0.0; 9];
    let mut d_off = [0.0; 3];

    // Go to address
    io.seek(SeekFrom::Current(offset as i64))?;

    for i in d_mat.iter_mut() {
        *i = io.read_s15f16()?;
    }
    for i in d_off.iter_mut() {
        *i = io.read_s15f16()?;
    }

    match Stage::alloc_matrix(3, 3, &d_mat, &d_off) {
        Some(val) => Ok(val),
        None => Err(ErrorKind::InvalidData.into()),
    }
}

fn read_clut(
    context: &mut Context,
    io: &mut dyn IOHandler,
    offset: usize,
    input_channels: u32,
    output_channels: u32,
) -> Result<Stage> {
    let mut grid_points_8 = [0u8; MAX_CHANNELS]; // Number of grid points in each dimension.
    let mut grid_points = [0u32; MAX_CHANNELS];

    // Go to address
    io.seek(SeekFrom::Current(offset as i64))?;
    io.read(&mut grid_points_8)?;

    for i in 0..MAX_CHANNELS {
        if grid_points_8[i] == 1 {
            return Err(ErrorKind::InvalidData.into()); // Impossible value, 0 for no CLUT and then 2 at least
        }
        grid_points[i] = grid_points_8[i] as u32;
    }

    let precision = io.read_u8()?;

    for _ in 0..3 {
        _ = io.read_u8()?;
    }

    let mut clut = match Stage::alloc_clut_16bit_granular(
        &grid_points,
        input_channels,
        output_channels,
        &[],
    ) {
        Some(clut) => clut,
        None => return Err(ErrorKind::InvalidData.into()),
    };
    let data = clut.data.as_mut().unwrap();
    let data = data.downcast_mut::<StageClutData>().unwrap();

    // Precision can be 1 or 2 bytes
    match precision {
        1 => {
            let tab: &mut Box<[u16]> = data.tab.as_u16_mut().unwrap();
            for i in 0..tab.len() {
                let v = io.read_u8()?;
                tab[i] = from_8_to_16(v);
            }
        }
        2 => {
            let tab: &mut Box<[u16]> = data.tab.as_u16_mut().unwrap();
            io.read_u16_array(tab)?;
        }
        _ => {
            context.signal_error(
                ErrorCode::UnknownExtension,
                format!("Unknown precision of '{}'", precision),
            );
            return Err(ErrorKind::InvalidData.into());
        }
    }

    Ok(clut)
}

fn write_16bit_tables(io: &mut dyn IOHandler, tables: &StageToneCurveData) -> Result<()> {
    let num_entries = tables.curves[0].table16.len();

    for curve in tables.curves.iter() {
        for j in 0..num_entries {
            let val = curve.table16[j];
            io.write_u16(val)?;
        }
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

pub(crate) mod chromaticity_tag;
pub(crate) mod colorant_order;
pub(crate) mod colorant_table;
pub(crate) mod curve_tag;
