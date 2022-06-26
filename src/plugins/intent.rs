use std::sync::Arc;

use crate::{
    state::Context,
    types::{Pipeline, Profile, Signature},
};

pub type IntentFn = fn(
    context: &mut Context,
    num_profiles: usize,
    intents: &[Signature],
    profiles: &[Profile],
    bpc: &[bool],
    adaption_states: &[f64],
    flags: u32,
) -> Arc<Pipeline>;
