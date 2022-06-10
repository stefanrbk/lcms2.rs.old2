use std::sync::{Arc, Mutex};

use crate::{io::IOHandler, state::Context, types::Signature};

pub type TagTypeReader = fn(
    handler: &TypeHandler,
    io: &dyn IOHandler,
    num_items: &mut u32,
    size_of_tag: usize,
) -> Option<Box<[u8]>>;

pub type TagTypeWriter =
    fn(handler: &TypeHandler, io: &dyn IOHandler, ptr: &[u8], num_items: usize) -> Option<()>;

pub struct TypeHandler {
    signature: Signature,
    context: Arc<Mutex<Context>>,
    icc_version: u32,
    read: TagTypeReader,
    write: TagTypeWriter,
}
