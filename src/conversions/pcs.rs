use crate::{
    math::quick_saturate_word,
    types::{CIELab, CIExyY, CIEXYZ},
};

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

#[inline(always)]
fn L_to_float2(v: u16) -> f64 {
    v as f64 / 652.800
}

#[inline(always)]
fn ab_to_float2(v: u16) -> f64 {
    (v as f64 / 256.0) - 128.0
}

#[inline(always)]
fn L_to_fix2(v: f64) -> u16 {
    quick_saturate_word(v * 652.8)
}

#[inline(always)]
fn ab_to_fix2(v: f64) -> u16 {
    quick_saturate_word((v + 128.0) * 256.0)
}

#[inline(always)]
fn L_to_float4(v: u16) -> f64 {
    v as f64 / 655.35
}

#[inline(always)]
fn ab_to_float4(v: u16) -> f64 {
    (v as f64 / 257.0) - 128.0
}

#[inline(always)]
pub fn Lab_encoded_to_float_v2(w: [u16; 3]) -> CIELab {
    CIELab {
        L: L_to_float2(w[0]),
        a: ab_to_float2(w[1]),
        b: ab_to_float2(w[2]),
    }
}

#[inline(always)]
pub fn Lab_encoded_to_float(w: [u16; 3]) -> CIELab {
    CIELab {
        L: L_to_float4(w[0]),
        a: ab_to_float4(w[1]),
        b: ab_to_float4(w[2]),
    }
}

#[inline(always)]
fn clamp_L_double_v2(v: f64) -> f64 {
    const MAX: f64 = (0xFFFF as f64 * 100.0) / 0xFF00 as f64;

    v.clamp(0.0, MAX)
}

#[inline(always)]
fn clamp_ab_double_v2(v: f64) -> f64 {
    v.clamp(CIELab::MIN_ENCODEABLE_ab2, CIELab::MAX_ENCODEABLE_ab2)
}

#[inline(always)]
pub fn float_to_Lab_encoded_v2(mut f: CIELab) -> [u16; 3] {
    f.L = clamp_L_double_v2(f.L);
    f.a = clamp_ab_double_v2(f.a);
    f.b = clamp_ab_double_v2(f.b);

    [L_to_fix2(f.L), ab_to_fix2(f.a), ab_to_fix2(f.b)]
}

#[inline(always)]
fn clamp_L_double_v4(v: f64) -> f64 {
    v.clamp(0.0, 100.0)
}

#[inline(always)]
fn clamp_ab_double_v4(v: f64) -> f64 {
    v.clamp(CIELab::MIN_ENCODEABLE_ab4, CIELab::MAX_ENCODEABLE_ab4)
}

#[inline(always)]
fn L_to_fix4(v: f64) -> u16 {
    quick_saturate_word(v * 655.35)
}

#[inline(always)]
fn ab_to_fix4(v: f64) -> u16 {
    quick_saturate_word((v + 128.0) * 257.0)
}

#[inline(always)]
pub fn float_to_Lab_encoded(mut f: CIELab) -> [u16; 3] {
    f.L = clamp_L_double_v4(f.L);
    f.a = clamp_ab_double_v4(f.a);
    f.b = clamp_ab_double_v4(f.b);

    [L_to_fix4(f.L), ab_to_fix4(f.a), ab_to_fix4(f.b)]
}
