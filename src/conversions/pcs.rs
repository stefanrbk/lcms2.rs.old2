use crate::types::{CIEXYZ, CIExyY};

#[inline(always)]
pub fn XYZ_to_xyY(source: CIEXYZ) -> CIExyY {
    let i_sum = 1.0/(source.X + source.Y + source.Z);

    CIExyY { x: source.X * i_sum, y: source.Y * i_sum, Y: source.Y }
}

#[inline(always)]
pub fn xyY_to_XYZ(source: CIExyY) -> CIEXYZ {
    CIEXYZ { 
        X: (source.x / source.y) * source.Y,
        Y: source.Y,
        Z: ((1.0 - source.x - source.y) / source.y) * source.Y
    }
}
