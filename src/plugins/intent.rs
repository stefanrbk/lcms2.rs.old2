use std::{fmt::Debug, sync::Arc};

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
pub type IntentsList = Vec<IntentsListItem>;
#[derive(Clone)]
pub struct IntentsListItem {
    pub intent: Signature,
    pub description: String,
    pub link: IntentFn,
}
impl Debug for IntentsListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IntentListItem")
            .field("intent", &self.intent)
            .field("description", &self.description)
            .field("link", &"[Function Ptr]")
            .finish()
    }
}
