#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub(crate) fn point_square_distance(&self, rhs: &Self) -> f64 {
        (self.x - rhs.x).powi(2) + (self.y - rhs.y).powi(2) + (self.z - rhs.z).powi(2)
    }
}
