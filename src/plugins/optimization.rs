use std::fmt::Debug;

use crate::types::{Pipeline, Signature};

pub type OPToptimizeFn =
    fn(lut: &mut Box<[Pipeline]>, intent: Signature) -> Option<(Signature, Signature, u32)>;
pub type OptimizationCollection = Vec<OptimizationCollectionItem>;

#[derive(Clone)]
pub struct OptimizationCollectionItem {
    pub optimize_ptr: OPToptimizeFn,
}

impl Debug for OptimizationCollectionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OptimizationCollectionItem")
            .field("optimize_ptr", &"[Function Ptr]")
            .finish()
    }
}
