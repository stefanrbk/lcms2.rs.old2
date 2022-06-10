use std::sync::{Arc, Mutex};

use crate::{state::Context, plugins::InterpParams};

use super::{Signature, ToneCurve};

pub type StageEvalFn = fn(r#in: &[f32], out: &mut [f32], mpe: &Stage);
pub struct Stage {
    context: Arc<Mutex<Context>>,
    r#type: Signature,
    implements: Signature,
    input_channels: u32,
    output_channels: u32,
    eval: StageEvalFn,
    data: Arc<Box<[u8]>>,
}

pub struct StageToneCurveData {
    curves: Arc<Vec<ToneCurve>>,
}
pub struct StageMatrixData {
    double: Box<[f64]>,
    offset: Box<[f64]>,
}
pub enum Tab {
    U16(Box<[u16]>),
    F32(Box<[f32]>),
}
pub struct StageClutData {
    tab: Tab,
    params: Box<[InterpParams]>,
}

pub enum PipelineEvalFn {
    U16(fn(r#in: &[u16], out: &mut [u16], data: Box<[u8]>)),
    Float(fn(r#in: &[f32], out: &mut [f32], data: Box<[u8]>)),
}
pub struct Pipeline {
    elements: Vec<Arc<Stage>>,
    input_channels: u32,
    output_channels: u32,
    data: Arc<Box<[u8]>>,
    eval: PipelineEvalFn,
    context: Arc<Mutex<Context>>,
    save_as_8_bits: bool,
}
