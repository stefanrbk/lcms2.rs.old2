#![allow(non_snake_case)]
use std::{fmt::Debug, io::{Result, Error, ErrorKind}};

use crate::{
    io::IOHandler,
    types::{signatures::tag_type, Signature, CIEXYZ, CIExyYTriple, CIExyY},
};

pub type TagTypeList = Vec<TypeHandler>;

pub type TagTypeReader = fn(
    handler: &TypeHandler,
    io: &mut dyn IOHandler,
    size_of_tag: usize,
) -> Result<(usize, Box<[u8]>)>;

pub type TagTypeWriter =
    fn(handler: &TypeHandler, io: &mut dyn IOHandler, ptr: &[u8], num_items: usize) -> Result<()>;

pub type TypeDecider = fn(version: f64, data: &Box<[u8]>) -> Signature;

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
        io: &mut dyn IOHandler,
        size_of_tag: usize,
    ) -> Result<(usize, Box<[u8]>)> {
        let mut chans = io.read_u16()?;

        // Let's recover from a bug introduced in early versions of the original lcms1 
        if chans == 0 && size_of_tag == 32 {
            _ = io.read_u16();
            chans = io.read_u16()?;
        }

        if chans != 3 {
            return Err(Error::from(ErrorKind::InvalidData))
        }
        _ = io.read_u16()?;
        let red = CIExyY {x: io.read_s15f16()?, y: io.read_s15f16()?, Y: 1.0 };
        let green = CIExyY { x: io.read_s15f16()?, y: io.read_s15f16()?, Y: 1.0 };
        let blue = CIExyY { x: io.read_s15f16()?, y: io.read_s15f16()?, Y: 1.0 };

        Ok((1, Box::new(CIExyYTriple { red, green, blue }.to_bytes())))
    }
    pub fn XYZ_read(
        &self,
        io: &mut dyn IOHandler,
        _size_of_tag: usize,
    ) -> Result<(usize, Box<[u8]>)> {
        Ok((1, Box::new(io.read_xyz()?.to_bytes())))
    }

    pub fn chromaticity_write(&self, io: &mut dyn IOHandler, ptr: &[u8], _num_items: usize) -> Result<()> {
        let value = CIExyYTriple::from_bytes(ptr);

        io.write_u16(3)?;
        io.write_u16(0)?;

        save_one_chromaticity(io, value.red.x, value.red.y)?;
        save_one_chromaticity(io, value.green.x, value.green.y)?;
        save_one_chromaticity(io, value.blue.x, value.blue.y)
    }
    pub fn XYZ_write(&self, io: &mut dyn IOHandler, ptr: &[u8], _num_items: usize) -> Result<()> {
        io.write_xyz(CIEXYZ::from_bytes(ptr))
    }

    pub fn decide_XYZ(_version: f64, _data: &Box<[u8]>) -> Signature {
        tag_type::XYZ
    }
}
fn save_one_chromaticity(io: &mut dyn IOHandler, x: f64, y: f64) -> Result<()> {
    io.write_s15f16(x)?;
    io.write_s15f16(y)
}
