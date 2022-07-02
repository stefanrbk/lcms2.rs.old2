use crate::{plugins::{TagTypeList, TypeHandler}, types::Signature};

#[derive(Clone, Debug, Default)]
pub struct TagTypePluginChunk {
    pub tag_types: TagTypeList,
}

impl TagTypePluginChunk {
    pub fn add(&mut self, handler: TypeHandler) {
        self.tag_types.push(handler);
    }

    pub fn get_handler(&self, sig: Signature) -> Option<&TypeHandler> {
        for pt in self.tag_types.iter() {
            if pt.signature == sig {
                return Some(pt);
            }
        }
        None
    }

    pub fn get_handler_mut(&mut self, sig: Signature) -> Option<&TypeHandler> {
        for pt in self.tag_types.iter_mut() {
            if pt.signature == sig {
                return Some(pt);
            }
        }
        None
    }
}
