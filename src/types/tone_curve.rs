use crate::plugins::{InterpParams, ParametricCurveEvaluator};

use super::CurveSegment;

pub struct ToneCurve {
    interp_params: InterpParams,
    num_segments: usize,
    segments: Box<[CurveSegment]>,
    seg_interp: Box<[Box<[InterpParams]>]>,
    evals: Option<Box<[ParametricCurveEvaluator]>>,
    table16: Option<Box<[u16]>>,
}
