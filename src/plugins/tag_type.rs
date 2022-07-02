use std::fmt::Debug;

use crate::{io::IOHandler, state::Context, types::Signature};

pub type TagTypeList = Vec<TypeHandler>;

pub type TagTypeReader = fn(
    context: &mut Context,
    handler: &TypeHandler,
    io: &dyn IOHandler,
    num_items: &mut u32,
    size_of_tag: usize,
) -> Option<Box<[u8]>>;

pub type TagTypeWriter = fn(
    context: &mut Context,
    handler: &TypeHandler,
    io: &dyn IOHandler,
    ptr: &[u8],
    num_items: usize,
) -> Option<()>;

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
