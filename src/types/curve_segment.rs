pub struct CurveSegment {
    pub(crate) x0: f32,
    pub(crate) x1: f32,
    pub(crate) r#type: i32,
    pub(crate) params: [f64; 10],
    pub(crate) sampled_points: Vec<f32>,
}
