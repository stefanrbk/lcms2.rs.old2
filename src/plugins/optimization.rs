use std::sync::Arc;

use crate::types::{Pipeline, Signature};

pub type OPToptimizeFn = fn(lut: &mut Box<[Pipeline]>, intent: Signature) -> Option<(Signature, Signature, u32)>;
