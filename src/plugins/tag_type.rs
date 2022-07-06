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
        signatures::tag_type,
        CIExyY, CIExyYTriple, DateTimeNumber, ICCData, NamedColor, NamedColorList, Signature,
        ToneCurve, CIEXYZ, MAX_CHANNELS,
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
