use crate::types::Signature;

pub type TagTypeDecoder = fn(icc_version: f64, data: &[u8]) -> Signature;

pub struct TagDescriptor {
    element_count: usize,
    supported_types: Vec<Signature>,
    decide_type: TagTypeDecoder,
}
