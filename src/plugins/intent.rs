use std::sync::{Arc, Mutex};

use crate::{
    state::Context,
    types::{Pipeline, Profile, Signature},
};

pub type IntentFn = fn(
    context: Arc<Mutex<Context>>,
    num_profiles: usize,
    intents: &[Signature],
    profiles: &[Profile],
    bpc: &[bool],
    adaption_states: &[f64],
    flags: u32,
) -> Arc<Pipeline>;
