#![allow(non_snake_case)]
use std::{fmt::Debug, io::Result};

use crate::{
    io::IOHandler,
    types::{signatures::tag_type, Signature, CIEXYZ},
};

pub type TagTypeList = Vec<TypeHandler>;

pub type TagTypeReader = fn(
    handler: &TypeHandler,
    io: &mut dyn IOHandler,
    size_of_tag: usize,
) -> Result<(usize, Box<[u8]>)>;

pub type TagTypeWriter =
    fn(handler: &TypeHandler, io: &mut dyn IOHandler, ptr: &[u8], num_items: usize) -> Result<()>;

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
    pub fn XYZ_read(
        &self,
        io: &mut dyn IOHandler,
        _size_of_tag: usize,
    ) -> Result<(usize, Box<[u8]>)> {
        Ok((1, Box::new(io.read_xyz()?.to_bytes())))
    }

    pub fn XYZ_write(&self, io: &mut dyn IOHandler, ptr: &[u8], _num_items: usize) -> Result<()> {
        io.write_xyz(CIEXYZ::from_bytes(ptr))
    }

    pub fn decide_XYZ(_version: f64, _data: Box<[u8]>) -> Signature {
        tag_type::XYZ
    }
}
