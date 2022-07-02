#![allow(non_snake_case)]
use std::{
    fmt::Debug,
    io::{ErrorKind, Result}, any::Any,
};

use crate::{
    io::IOHandler,
    state::{Context, ErrorCode},
    types::{signatures::tag_type, CIExyY, CIExyYTriple, Signature, CIEXYZ, MAX_CHANNELS, NamedColorList, NamedColor},
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
    pub fn colorant_tree_read(
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

        Ok((0, Box::new([0])))
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
fn save_one_chromaticity(io: &mut dyn IOHandler, x: f64, y: f64) -> Result<()> {
    io.write_s15f16(x)?;
    io.write_s15f16(y)
}
