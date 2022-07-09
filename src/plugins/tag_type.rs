#![allow(non_snake_case)]
use std::{
    any::Any,
    fmt::Debug,
    io::{ErrorKind, Result},
    mem::size_of,
};

use crate::{
    io::IOHandler,
    math::{f64_to_u8f8, u8f8_to_f64},
    state::{Context, ErrorCode},
    types::{
        signatures::{stage, tag_type},
        CIExyY, CIExyYTriple, DateTimeNumber, ICCData, Matrix, NamedColor, NamedColorList,
        Pipeline, Signature, Stage, StageClutData, StageLoc, StageMatrixData, StageToneCurveData,
        ToneCurve, Vec3, CIEXYZ, MAX_CHANNELS,
    },
};

pub type TagTypeList = Vec<TypeHandler>;

pub type TagTypeReader = fn(
    handler: &TypeHandler,
    context: &mut Context,
    io: &mut dyn IOHandler,
    size_of_tag: usize,
) -> Result<(usize, Box<dyn Any>)>;

pub type TagTypeWriter = fn(
    handler: &TypeHandler,
    context: &mut Context,
    io: &mut dyn IOHandler,
    ptr: Box<dyn Any>,
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
    pub fn chromaticity_read(
        &self,
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
    pub fn colorant_order_read(
        &self,
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
    pub fn colorant_table_read(
        &self,
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
    pub fn curve_read(
        &self,
        context: &mut Context,
        io: &mut dyn IOHandler,
        _size_of_tag: usize,
    ) -> Result<(usize, Box<dyn Any>)> {
        let count = io.read_u32()?;

        match count {
            // Linear
            0 => {
                let single_gamma = 1.0;
                if let Some(new_gamma) = ToneCurve::build_parametric(context, 1, &[single_gamma]) {
                    return Ok((1, Box::new(new_gamma)));
                }
                return Err(ErrorKind::InvalidData.into());
            }
            // Specified as the exponent of gamma function
            1 => {
                let single_gamma_fixed = io.read_u16()?;
                let single_gamma = u8f8_to_f64(single_gamma_fixed);

                if let Some(new_gamma) = ToneCurve::build_parametric(context, 1, &[single_gamma]) {
                    return Ok((1, Box::new(new_gamma)));
                }
                return Err(ErrorKind::InvalidData.into());
            }
            // Curve
            _ => {
                // This is to prevent bad guys from doing bad things
                if count > 0x7FFF {
                    return Err(ErrorKind::InvalidData.into());
                }
                if let Some(mut new_gamma) =
                    ToneCurve::build_tabulated_16(context, count as usize, &[0u16; 0])
                {
                    let mut buf = vec![0u16; count as usize];
                    io.read_u16_array(buf.as_mut_slice())?;
                    new_gamma.table16 = buf.into_boxed_slice();
                }
                return Err(ErrorKind::InvalidData.into());
            }
        }
    }
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

        // Get input tables
        read_16bit_tables(
            context,
            io,
            &mut new_lut,
            output_channels as u32,
            output_entries as u32,
        )?;

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

    /// ptr MUST be a &Box<CIExyYTriple>
    pub fn chromaticity_write(
        &self,
        _context: &mut Context,
        io: &mut dyn IOHandler,
        ptr: Box<dyn Any>,
        _num_items: usize,
    ) -> Result<()> {
        let value = ptr.downcast::<CIExyYTriple>().unwrap();

        io.write_u16(3)?;
        io.write_u16(0)?;

        save_one_chromaticity(io, value.red.x, value.red.y)?;
        save_one_chromaticity(io, value.green.x, value.green.y)?;
        save_one_chromaticity(io, value.blue.x, value.blue.y)
    }
    /// ptr MUST be &Box<Vec<u8>>
    pub fn colorant_order_write(
        &self,
        _context: &mut Context,
        io: &mut dyn IOHandler,
        ptr: Box<dyn Any>,
        _num_items: usize,
    ) -> Result<()> {
        let ptr = ptr.downcast::<Vec<u8>>().unwrap();
        let len = ptr.len() as u32;

        io.write_u32(len)?;

        for value in ptr.iter() {
            io.write_u8(*value)?;
        }
        Ok(())
    }
    /// ptr MUST be &Box<NamedColorList>
    pub fn colorant_table_write(
        &self,
        _context: &mut Context,
        io: &mut dyn IOHandler,
        ptr: Box<dyn Any>,
        _num_items: usize,
    ) -> Result<()> {
        let ptr = ptr.downcast::<NamedColorList>().unwrap();
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
    /// ptr MUST be &Box<ToneCurve>
    pub fn curve_write(
        &self,
        _context: &mut Context,
        io: &mut dyn IOHandler,
        ptr: Box<dyn Any>,
        _num_items: usize,
    ) -> Result<()> {
        let ptr = ptr.downcast::<ToneCurve>().unwrap();

        if ptr.segments.len() == 1 && ptr.segments[0].segment.r#type == 1 {
            // Single gamma, preserve number
            let single_gamma_fixed = f64_to_u8f8(ptr.segments[0].segment.params[0]);

            io.write_u32(1)?;
            io.write_u16(single_gamma_fixed)?;
            return Ok(());
        }
        io.write_u32(ptr.table16.len() as u32)?;
        io.write_u16_array(&ptr.table16)
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
        if num_tab_size == u32::MAX{
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

    pub fn decide_curve(version: f64, data: &Box<dyn Any>) -> Signature {
        let curve = data.downcast_ref::<ToneCurve>().unwrap();
        if version < 4.0 {
            return tag_type::CURVE;
        }
        // Only 1-segment curves can be saved as parametric
        if curve.segments.len() != 1 {
            return tag_type::CURVE;
        }
        // Only non-inverted curves
        if curve.segments[0].segment.r#type < 0 {
            return tag_type::CURVE;
        }
        // Only ICC parametric curves
        if curve.segments[0].segment.r#type > 5 {
            return tag_type::CURVE;
        }
        tag_type::PARAMETRIC_CURVE
    }
    pub fn decide_XYZ(_version: f64, _data: &Box<dyn Any>) -> Signature {
        tag_type::XYZ
    }
}
fn save_one_chromaticity(io: &mut dyn IOHandler, x: f64, y: f64) -> Result<()> {
    io.write_s15f16(x)?;
    io.write_s15f16(y)
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
