pub(crate) mod consts;

use crate::coordinate::Coordinate;

pub use consts::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Face {
    pub face: isize,
    pub coord: Coordinate,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FaceOrient {
    pub face: isize,
    pub translate: Coordinate,
    pub ccw_rot_60: isize,
}

/// Digit representing overage type
pub enum Overage {
    /// No overage (on original face)
    NoOverage,
    /// On face edge (only occurs on substrate grids)
    FaceEdge,
    /// Overage on new face interior
    NewFace,
}
