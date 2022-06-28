use std::sync::Arc;

use super::{Mlu, ProfileID, Signature};

pub struct SequenceDescriptor {
    device_mfg: Signature,
    device_model: Signature,
    attributes: u64,
    technology: Signature,
    profile_id: ProfileID,
    manufacturer: Arc<Mlu>,
    model: Arc<Mlu>,
    description: Arc<Mlu>,
}

pub struct Sequence {
    seq: Vec<SequenceDescriptor>,
}
// &mut Context must be passed in for all functions involving Sequence
