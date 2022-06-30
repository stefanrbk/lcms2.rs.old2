use crate::types::{CIELab, CIExyY, CIEXYZ};

use super::whitepoint::D50_XYZ;

#[inline(always)]
pub fn XYZ_to_xyY(source: CIEXYZ) -> CIExyY {
    let i_sum = 1.0 / (source.X + source.Y + source.Z);

    CIExyY {
        x: source.X * i_sum,
        y: source.Y * i_sum,
        Y: source.Y,
    }
}

#[inline(always)]
pub fn xyY_to_XYZ(source: CIExyY) -> CIEXYZ {
    CIEXYZ {
        X: (source.x / source.y) * source.Y,
        Y: source.Y,
        Z: ((1.0 - source.x - source.y) / source.y) * source.Y,
    }
}

#[inline(always)]
fn f(t: f64) -> f64 {
    let lim = 24.0 / 116.0;
    let lim = lim * lim * lim;

    if t <= lim {
        (841.0 / 108.0) * t + (16.0 / 116.0)
    } else {
        t.powf(1.0 / 3.0)
    }
}

#[inline(always)]
fn f_1(t: f64) -> f64 {
    let lim = 24.0 / 116.0;

    if t <= lim {
        (108.0 / 841.0) * (t - (16.0 / 116.0))
    } else {
        t.powi(3)
    }
}

#[inline(always)]
pub fn XYZ_to_Lab(whitepoint: Option<CIEXYZ>, xyz: CIEXYZ) -> CIELab {
    let whitepoint = whitepoint.unwrap_or(D50_XYZ);

    let fx = f(xyz.X / whitepoint.X);
    let fy = f(xyz.Y / whitepoint.Y);
    let fz = f(xyz.Z / whitepoint.Z);

    CIELab {
        L: 116.0 * fy - 16.0,
        a: 500.0 * (fx - fy),
        b: 200.0 * (fy - fz),
    }
}

#[inline(always)]
pub fn Lab_to_XYZ(whitepoint: Option<CIEXYZ>, lab: CIELab) -> CIEXYZ {
    let whitepoint = whitepoint.unwrap_or(D50_XYZ);

    let y = (lab.L + 16.0) / 116.0;
    let x = y + 0.002 * lab.a;
    let z = y - 0.005 * lab.b;

    CIEXYZ {
        X: f_1(x) * whitepoint.X,
        Y: f_1(y) * whitepoint.Y,
        Z: f_1(z) * whitepoint.Z,
    }
}
