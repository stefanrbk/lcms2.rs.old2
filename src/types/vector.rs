pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }
    pub fn minus(&self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
    pub fn cross(&self, other: Self) -> Self {
        Self(
            self.1 * other.2 - other.1 * self.2,
            self.2 * other.0 - other.2 * self.0,
            self.0 * other.1 - other.0 * self.1,
        )
    }
    pub fn dot(&self, other: Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
    pub fn length(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }
    pub fn distance(&self, other: Self) -> f64 {
        self.minus(other).length()
    }
}
