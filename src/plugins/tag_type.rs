#![allow(non_snake_case)]
use std::{
    any::Any,
    fmt::Debug,
    io::{ErrorKind, Result, SeekFrom},
};

use crate::{
    io::IOHandler,
    math::from_8_to_16,
    state::{Context, ErrorCode},
    types::{signatures::tag_type, Signature, Stage, StageClutData, ToneCurve, MAX_CHANNELS},
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

pub fn read_type_base(io: &mut dyn IOHandler) -> Result<Signature> {
    let base = io.read_u32()?;
    _ = io.read_u32()?;

    Ok(Signature::from(base))
}
pub fn write_type_base(io: &mut dyn IOHandler, sig: Signature) -> Result<()> {
    io.write_u32(sig.into())?;
    io.write_u32(0)
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
fn read_embedded_curve(context: &mut Context, io: &mut dyn IOHandler) -> Result<ToneCurve> {
    let base_type = read_type_base(io)?;

    match base_type {
        tag_type::CURVE => {
            let (count, result) = curve_type::read(context, io, 0)?;
            if count != 1 {
                return Err(ErrorKind::InvalidData.into());
            }
            match result.downcast::<ToneCurve>() {
                Ok(result) => Ok(*result),
                Err(_) => Err(ErrorKind::InvalidData.into()),
            }
        }
        tag_type::PARAMETRIC_CURVE => {
            todo!();
            // repeat tag_type::CURVE, but with parametric_curve_type::read
        }
        _ => {
            let error = format!("Unknown curve type '{:?}'", base_type);
            context.signal_error(ErrorCode::UnknownExtension, error);
            Err(ErrorKind::InvalidData.into())
        }
    }
}
fn read_set_of_curves(
    context: &mut Context,
    io: &mut dyn IOHandler,
    offset: usize,
    num_curves: usize,
) -> Result<Stage> {
    let mut curves: [ToneCurve; MAX_CHANNELS] = Default::default();

    if num_curves > MAX_CHANNELS {
        return Err(ErrorKind::InvalidInput.into());
    }
    io.seek(SeekFrom::Current(offset as i64))?;

    for i in 0..num_curves {
        curves[i] = read_embedded_curve(context, io)?;
        io.read_alignment()?;
    }

    if let Some(lin) = Stage::alloc_tone_curves(num_curves as u32, &curves) {
        Ok(lin)
    } else {
        Err(ErrorKind::InvalidData.into())
    }
}

pub(crate) mod chromaticity_type;
pub(crate) mod colorant_order_type;
pub(crate) mod colorant_table_type;
pub(crate) mod curve_type;
pub(crate) mod data_type;
pub(crate) mod date_time_type;
pub(crate) mod lut16_type;
pub(crate) mod lut8_type;
pub(crate) mod lut_a_to_b_type;
pub(crate) mod xyz_type;
