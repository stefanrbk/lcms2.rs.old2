pub struct CurveSegment {
    x0: f32,
    x1: f32,
    r#type: i32,
    params: [f64; 10],
    sampled_points: Vec<f32>,
}
