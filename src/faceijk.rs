use crate::{error::H3ErrorCode, CoordIJK, LatLng};

pub const IJ: u8 = 1;
pub const KI: u8 = 2;
pub const JK: u8 = 3;
pub const IVALID_FACE: i8 = -1;

/// Face number and ijk coordinates on that face-centered coordinate.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FaceIJK {
    pub face: i32,
    pub coord: CoordIJK,
}

/// Information to transform into an adjacent face IJK system.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FaceOrientIJK {
    pub face: i32,
    pub translate: CoordIJK,
    pub ccw_rot60: i32,
}

/// Digit representing overage type.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Overage {
    NoOverage,
    FaceEdge,
    NewFace,
}

impl TryInto<LatLng> for FaceIJK {
    type Error = H3ErrorCode;

    fn try_into(self) -> Result<LatLng, Self::Error> {
        (&self).try_into()
    }
}

impl TryInto<LatLng> for &FaceIJK {
    type Error = H3ErrorCode;

    fn try_into(self) -> Result<LatLng, Self::Error> {
        //
    }
}
