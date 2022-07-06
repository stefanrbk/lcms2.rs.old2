use crate::{
    plugins::{InterpParams, ParametricCurveEvaluator, ParametricCurves, MINUS_INF, PLUS_INF, default_eval_parametric_fn},
    state::{Context, ErrorCode},
};

use super::CurveSegment;

#[derive(Default)]
pub struct ToneCurve {
    pub(crate) interp_params: InterpParams,
    pub(crate) segments: Box<[ToneCurveSegment]>,
    pub(crate) table16: Box<[u16]>,
}

#[derive(Clone)]
pub struct ToneCurveSegment {
    pub(crate) segment: CurveSegment,
    pub(crate) seg_interp: Box<[InterpParams]>,
    pub(crate) eval: ParametricCurveEvaluator,
}
impl Default for ToneCurveSegment {
    fn default() -> Self {
        Self {
            segment: Default::default(),
            seg_interp: Default::default(),
            eval: default_eval_parametric_fn,
        }
    }
}

impl ToneCurve {
    pub fn build_tabulated_16(context: &mut Context, num_entries: usize, values: &[u16]) -> Option<Self> {
        ParametricCurves::allocate_tone_curve_struct(context, num_entries, 0, &[CurveSegment::default(); 0], values)
    }
    pub fn build_parametric(context: &mut Context, r#type: i32, params: &[f64]) -> Option<Self> {
        let c = ParametricCurves::get_collection_by_type(context, r#type);

        if c.is_none() {
            let error = format!("Invalid parametric curve type {}", r#type);
            context.signal_error(ErrorCode::UnknownExtension, error);
            return None;
        }
        let c = c.unwrap();

        let mut result_params = [0.0; 10];
        let count = c.0.curves[c.1].parameter_count as usize;

        result_params[0..count].copy_from_slice(&params[0..count]);

        let result = CurveSegment {
            x0: MINUS_INF as f32,
            x1: PLUS_INF as f32,
            r#type,
            params: result_params,
            sampled_points: Default::default(),
        };

        Self::build_segmented(context, &[result])
    }
    pub fn build_segmented(context: &mut Context, segments: &[CurveSegment]) -> Option<Self> {
        let mut num_grid_points: usize = 4096;

        // Optimization for identity curves
        if segments.len() == 1 && segments[0].r#type == 1 {
            num_grid_points = entries_by_gamma(segments[0].params[0]);
        }

        None
    }
}
fn entries_by_gamma(gamma: f64) -> usize {
    if (gamma - 1.0).abs() < 0.001 {
        return 2;
    }
    4096
}
