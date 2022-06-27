use std::fmt::Debug;

use crate::types::Signature;

pub type TagTypeDecoder = fn(icc_version: f64, data: &[u8]) -> Signature;
pub type TagList = Vec<TagListItem>;

#[derive(Clone, Debug)]
pub struct TagListItem {
    pub signature: Signature,
    pub descriptor: TagDescriptor,
}

#[derive(Clone)]
pub struct TagDescriptor {
    element_count: usize,
    supported_types: Vec<Signature>,
    decide_type: TagTypeDecoder,
}

impl Debug for TagDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TagDescriptor")
            .field("element_count", &self.element_count)
            .field("supported_types", &self.supported_types)
            .field("decide_type", &"[Function Ptr]")
            .finish()
    }
}
