use crate::MATRIX_DET_TOLERANCE;

pub type ParametricCurveEvaluator = fn(curve_type: i32, params: &[f64], r: f64) -> f64;
pub const MAX_NODES_IN_CURVE: usize = 4097;
pub const MINUS_INF: f64 = -1e22f64;
pub const PLUS_INF: f64 = 1e22f64;

pub type ParametricCurvesCollection = Vec<ParametricCurves>;

#[derive(Clone)]
pub struct ParametricCurves {
    pub curves: Vec<Curve>,
    pub evaluator: ParametricCurveEvaluator,
}
#[derive(Clone)]
pub struct Curve {
    pub function_curve_id: i32,
    pub parameter_count: u32,
}
impl Default for ParametricCurves {
    fn default() -> Self {
        let mut curves = Vec::with_capacity(10);
        curves.push(Curve {
            function_curve_id: 1,
            parameter_count: 1,
        });
        curves.push(Curve {
            function_curve_id: 2,
            parameter_count: 3,
        });
        curves.push(Curve {
            function_curve_id: 3,
            parameter_count: 4,
        });
        curves.push(Curve {
            function_curve_id: 4,
            parameter_count: 5,
        });
        curves.push(Curve {
            function_curve_id: 5,
            parameter_count: 7,
        });
        curves.push(Curve {
            function_curve_id: 6,
            parameter_count: 4,
        });
        curves.push(Curve {
            function_curve_id: 7,
            parameter_count: 5,
        });
        curves.push(Curve {
            function_curve_id: 8,
            parameter_count: 5,
        });
        curves.push(Curve {
            function_curve_id: 108,
            parameter_count: 1,
        });
        curves.push(Curve {
            function_curve_id: 109,
            parameter_count: 1,
        });

        Self {
            curves,
            evaluator: default_eval_parametric_fn,
        }
    }
}

impl ParametricCurves {
    pub fn is_in_set(&self, r#type: i32) -> Option<usize> {
        for i in 0..self.curves.len() {
            if r#type.abs() == self.curves[i].function_curve_id {
                return Some(i);
            }
        }
        None
    }
}

pub fn default_eval_parametric_fn(r#type: i32, params: &[f64], r: f64) -> f64 {
    match r#type {
        // Y = X ^ gamma
        1 => {
            let gamma = params[0];

            if r < 0.0 {
                if gamma.abs() - 1.0 < MATRIX_DET_TOLERANCE {
                    r
                } else {
                    0.0
                }
            } else {
                r.powf(gamma)
            }
        }
        // Type 1 Reversed: X = Y ^ ⅟gamma
        -1 => {
            let gamma = params[0];

            if r < 0.0 {
                if (gamma - 1.0).abs() < MATRIX_DET_TOLERANCE {
                    r
                } else {
                    0.0
                }
            } else {
                if gamma.abs() < MATRIX_DET_TOLERANCE {
                    PLUS_INF
                } else {
                    r.powf(1.0 / gamma)
                }
            }
        }
        // CIE 122-1966
        // Y = (aX + b)^Gamma  | X >= -b/a
        // Y = 0               | else
        2 => {
            let gamma = params[0];
            let a = params[1];
            let b = params[2];

            if a.abs() < MATRIX_DET_TOLERANCE {
                0.0
            } else {
                let disc = -b / a;
                if r >= disc {
                    let e = a * r + b;

                    if e > 0.0 {
                        e.powf(gamma)
                    } else {
                        0.0
                    }
                } else {
                    0.0
                }
            }
        }
        // Type 2 Reversed
        // X = (Y ^1/g - b) / a
        -2 => {
            let gamma = params[0];
            let a = params[1];
            let b = params[2];

            if gamma.abs() < MATRIX_DET_TOLERANCE || a.abs() < MATRIX_DET_TOLERANCE {
                0.0
            } else {
                let val = if r < 0.0 {
                    0.0
                } else {
                    (r.powf(1.0 / gamma) - b) / a
                };
                if val < 0.0 {
                    0.0
                } else {
                    val
                }
            }
        }
        // IEC 61966-3
        // Y = (aX + b)^Gamma | X <= -b/a
        // Y = c              | else
        3 => {
            let gamma = params[0];
            let a = params[1];
            let b = params[2];
            let c = params[3];

            if a.abs() < MATRIX_DET_TOLERANCE {
                0.0
            } else {
                let disc = (-b / a).max(0.0);

                if r >= disc {
                    let e = a * r + b;

                    if e > 0.0 {
                        e.powf(gamma) + c
                    } else {
                        0.0
                    }
                } else {
                    c
                }
            }
        }
        // Type 3 reversed
        // X=((Y-c)^1/g - b)/a      | (Y>=c)
        // X=-b/a                   | (Y<c)
        -3 => {
            let gamma = params[0];
            let a = params[1];
            let b = params[2];
            let c = params[3];

            if a.abs() < MATRIX_DET_TOLERANCE {
                0.0
            } else {
                if r >= c {
                    let e = r - c;

                    if e > 0.0 {
                        (e.powf(1.0 / gamma) - b) / a
                    } else {
                        0.0
                    }
                } else {
                    -b / a
                }
            }
        }
        // IEC 61966-2.1 (sRGB)
        // Y = (aX + b)^Gamma | X >= d
        // Y = cX             | X < d
        4 => {
            let gamma = params[0];
            let a = params[1];
            let b = params[2];
            let c = params[3];
            let d = params[4];

            if r >= d {
                let e = a * r + b;

                if e > 0.0 {
                    e.powf(gamma)
                } else {
                    0.0
                }
            } else {
                r * c
            }
        }
        // Type 4 reversed
        // X=((Y^1/g-b)/a)    | Y >= (ad+b)^g
        // X=Y/c              | Y< (ad+b)^g
        -4 => {
            let gamma = params[0];
            let a = params[1];
            let b = params[2];
            let c = params[3];
            let d = params[4];

            let e = a * d + b;
            let disc = if e < 0.0 { 0.0 } else { e.powf(gamma) };

            if r >= disc {
                if gamma.abs() < MATRIX_DET_TOLERANCE || a.abs() < MATRIX_DET_TOLERANCE {
                    0.0
                } else {
                    (r.powf(1.0 / gamma) - b) / a
                }
            } else {
                if c.abs() < MATRIX_DET_TOLERANCE {
                    0.0
                } else {
                    r / c
                }
            }
        }
        // Y = (aX + b)^Gamma + e | X >= d
        // Y = cX + f             | X < d
        5 => {
            let gamma = params[0];
            let a = params[1];
            let b = params[2];
            let c = params[3];
            let d = params[4];
            let e = params[5];
            let f = params[6];

            if r >= d {
                let _e = a * r + b;

                if _e > 0.0 {
                    _e.powf(gamma) + e
                } else {
                    e
                }
            } else {
                r * c + f
            }
        }
        // Reversed type 5
        // X=((Y-e)1/g-b)/a   | Y >=(ad+b)^g+e), cd+f
        // X=(Y-f)/c          | else
        -5 => {
            let gamma = params[0];
            let a = params[1];
            let b = params[2];
            let c = params[3];
            let d = params[4];
            let e = params[5];
            let f = params[6];

            let disc = c * d + f;
            if r >= disc {
                let e = r - e;
                if e < 0.0 {
                    0.0
                } else {
                    if gamma.abs() < MATRIX_DET_TOLERANCE || a.abs() < MATRIX_DET_TOLERANCE {
                        0.0
                    } else {
                        (e.powf(1.0 / gamma) - b) / a
                    }
                }
            } else {
                if c.abs() < MATRIX_DET_TOLERANCE {
                    0.0
                } else {
                    (r - f) / c
                }
            }
        }
        // Y = (a * X + b) ^ Gamma + c
        6 => {
            let gamma = params[0];
            let a = params[1];
            let b = params[2];
            let c = params[3];

            let e = a * r + b;

            if e < 0.0 {
                c
            } else {
                e.powf(gamma) + c
            }
        }
        // ((Y - c) ^1/Gamma - b) / a
        -6 => {
            let gamma = params[0];
            let a = params[1];
            let b = params[2];
            let c = params[3];

            if a.abs() < MATRIX_DET_TOLERANCE {
                0.0
            } else {
                let e = r - c;

                if e < 0.0 {
                    0.0
                } else {
                    (e.powf(1.0 / gamma) - b) / a
                }
            }
        }
        // Y = a * log (b * X^Gamma + c) + d
        7 => {
            let gamma = params[0];
            let a = params[1];
            let b = params[2];
            let c = params[3];
            let d = params[4];

            let e = b * r.powf(gamma) + c;
            if e <= 0.0 {
                d
            } else {
                a * e.log10() + d
            }
        }
        // (Y - d) / a = log(b * X ^Gamma + c)
        // pow(10, (Y-d) / a) = b * X ^Gamma + c
        // pow((pow(10, (Y-d) / a) - c) / b, 1/g) = X
        -7 => {
            let gamma = params[0];
            let a = params[1];
            let b = params[2];
            let c = params[3];
            let d = params[4];

            if gamma < MATRIX_DET_TOLERANCE || a < MATRIX_DET_TOLERANCE || b < MATRIX_DET_TOLERANCE
            {
                0.0
            } else {
                ((10.0f64.powf((r - d) / a) - c) / b).powf(1.0 / gamma)
            }
        }
        //Y = a * b^(c*X+d) + e
        8 => {
            let a = params[0];
            let b = params[1];
            let c = params[2];
            let d = params[3];
            let e = params[4];

            a * b.powf(c * r + d) + e
        }
        // Y = (log((y-e) / a) / log(b) - d ) / c
        -8 => {
            let a = params[0];
            let b = params[1];
            let c = params[2];
            let d = params[3];
            let e = params[4];

            let disc = r - e;

            if disc < 0.0 {
                0.0
            } else {
                if a.abs() < MATRIX_DET_TOLERANCE || c.abs() < MATRIX_DET_TOLERANCE {
                    0.0
                } else {
                    ((disc / a).log10() / b.log10() - d) / c
                }
            }
        }
        // S-Shaped: (1 - (1-x)^1/g)^1/g
        108 => {
            let g = params[0];
            if g.abs() < MATRIX_DET_TOLERANCE {
                0.0
            } else {
                (1.0 - (1.0 - r).powf(1.0 / g)).powf(1.0 / g)
            }
        }
        // 1 - (1 - y^g)^g
        -108 => {
            let g = params[0];

            1.0 - (1.0 - r.powf(g)).powf(g)
        }
        // Sigmoidals
        109 => sigmoid_factory(params[0], r),
        -109 => inverted_sigmoid_factory(params[0], r),
        _ => 0f64,
    }
}

#[inline]
fn sigmoid_base(k: f64, t: f64) -> f64 {
    (1.0 / (1.0 + (-k * t).exp())) - 0.5
}
#[inline]
fn inverted_sigmoid_base(k: f64, t: f64) -> f64 {
    -((1.0 / (t + 0.5)) - 1.0).log10() / k
}
#[inline]
fn sigmoid_factory(k: f64, t: f64) -> f64 {
    let correction = 0.5 / sigmoid_base(k, 1.0);

    correction * sigmoid_base(k, 2.0 * t - 1.0) + 0.5
}
#[inline]
fn inverted_sigmoid_factory(k: f64, t: f64) -> f64 {
    let correction = 0.5 / sigmoid_base(k, 1.0);

    (inverted_sigmoid_base(k, (t - 0.5) / correction) + 1.0) / 2.0
}
