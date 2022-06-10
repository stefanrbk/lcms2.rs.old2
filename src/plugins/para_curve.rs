pub type ParametricCurveEvaluator = fn(curve_type: i32, params: [f64; 10], r: f64) -> f64;
