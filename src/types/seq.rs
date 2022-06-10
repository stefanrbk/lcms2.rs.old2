use std::sync::{Arc, Mutex};

use crate::state::Context;

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
    context: Arc<Mutex<Context>>,
    seq: Vec<SequenceDescriptor>,
}
