use crate::MATRIX_DET_TOLERANCE;

use super::Vec3;

pub struct Matrix(pub Vec3, pub Vec3, pub Vec3);

impl Matrix {
    pub const IDENTITY: Self = Self(
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    );

    pub fn is_identity(&self) -> bool {
        close_enough(self.0 .0, Self::IDENTITY.0 .0)
            && close_enough(self.0 .1, Self::IDENTITY.0 .1)
            && close_enough(self.0 .2, Self::IDENTITY.0 .2)
            && close_enough(self.1 .0, Self::IDENTITY.1 .0)
            && close_enough(self.1 .1, Self::IDENTITY.1 .1)
            && close_enough(self.1 .2, Self::IDENTITY.1 .2)
            && close_enough(self.2 .0, Self::IDENTITY.2 .0)
            && close_enough(self.2 .1, Self::IDENTITY.2 .1)
            && close_enough(self.2 .2, Self::IDENTITY.2 .2)
    }

    pub fn product(&self, other: Self) -> Self {
        Self(
            Vec3::new(
                self.0 .0 * other.0 .0 + self.0 .1 * other.1 .0 + self.0 .2 * other.2 .0,
                self.0 .0 * other.0 .1 + self.0 .1 * other.1 .1 + self.0 .2 * other.2 .1,
                self.0 .0 * other.0 .2 + self.0 .1 * other.1 .2 + self.0 .2 * other.2 .2,
            ),
            Vec3::new(
                self.1 .0 * other.0 .0 + self.1 .1 * other.1 .0 + self.1 .2 * other.2 .0,
                self.1 .0 * other.0 .1 + self.1 .1 * other.1 .1 + self.1 .2 * other.2 .1,
                self.1 .0 * other.0 .2 + self.1 .1 * other.1 .2 + self.1 .2 * other.2 .2,
            ),
            Vec3::new(
                self.2 .0 * other.0 .0 + self.2 .1 * other.1 .0 + self.2 .2 * other.2 .0,
                self.2 .0 * other.0 .1 + self.2 .1 * other.1 .1 + self.2 .2 * other.2 .1,
                self.2 .0 * other.0 .2 + self.2 .1 * other.1 .2 + self.2 .2 * other.2 .2,
            ),
        )
    }

    pub fn inverse(&self) -> Option<Self> {
        let c0: f64 = self.1 .1 * self.2 .2 - self.1 .2 * self.2 .1;
        let c1: f64 = -self.1 .0 * self.2 .2 + self.1 .2 * self.2 .0;
        let c2: f64 = self.1 .0 * self.2 .1 - self.1 .1 * self.2 .0;

        let det: f64 = self.0 .0 * c0 + self.0 .1 * c1 + self.0 .2 * c2;

        if det.abs() < MATRIX_DET_TOLERANCE {
            return None;
        }

        Some(Self(
            Vec3::new(
                c0 / det,
                (self.0 .2 * self.2 .1 - self.0 .1 * self.2 .2) / det,
                (self.0 .1 * self.1 .2 - self.0 .2 * self.1 .1) / det,
            ),
            Vec3::new(
                c1 / det,
                (self.0 .0 * self.2 .2 - self.0 .2 * self.2 .0) / det,
                (self.0 .2 * self.1 .0 - self.0 .0 * self.1 .2) / det,
            ),
            Vec3::new(
                c2 / det,
                (self.0 .1 * self.2 .0 - self.0 .0 * self.2 .1) / det,
                (self.0 .0 * self.1 .1 - self.0 .1 * self.1 .0) / det,
            ),
        ))
    }

    pub fn solve(&self, other: Vec3) -> Option<Vec3> {
        if let Some(self_1) = self.inverse() {
            Some(self_1.eval(other))
        } else {
            None
        }
    }
    pub fn eval(&self, other: Vec3) -> Vec3 {
        Vec3(
            self.0 .0 * other.0 + self.0 .1 * other.1 + self.0 .2 * other.2,
            self.1 .0 * other.0 + self.1 .1 * other.1 + self.1 .2 * other.2,
            self.2 .0 * other.0 + self.2 .1 * other.1 + self.2 .2 * other.2,
        )
    }
}
impl Into<Matrix> for [f64; 9] {
    fn into(self) -> Matrix {
        Matrix(
            Vec3(self[0], self[1], self[2]),
            Vec3(self[3], self[4], self[5]),
            Vec3(self[6], self[7], self[8]),
        )
    }
}
impl Into<[f64; 9]> for Matrix {
    fn into(self) -> [f64; 9] {
        [
            self.0 .0, self.0 .1, self.0 .2, self.1 .0, self.1 .1, self.1 .2, self.2 .0, self.2 .1,
            self.2 .2,
        ]
    }
}

#[inline]
fn close_enough(a: f64, b: f64) -> bool {
    (b - a).abs() < (1.0 / 65535.0)
}
